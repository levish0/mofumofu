use crate::dto::follow::internal::delete::DeleteFollow;
use crate::entity::common::{ActionType, TargetType};
use crate::repository::follow::delete_follow::repository_delete_follow;
use crate::repository::system_events::log_event::repository_log_event;
use crate::repository::user::find_user_by_handle::repository_find_user_by_handle;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::ConnectionTrait;
use sea_orm::TransactionTrait;

pub async fn service_delete_follow_by_handle<C>(
    conn: &C,
    payload: DeleteFollow,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    let follower = repository_find_user_by_uuid(&txn, &payload.follower_id)
        .await?
        .ok_or(Errors::UserNotFound)?;
    let followee = repository_find_user_by_handle(&txn, &payload.followee_handle)
        .await?
        .ok_or(Errors::UserNotFound)?;

    let deleted = repository_delete_follow(&txn, follower.id, followee.id).await?;

    if deleted {
        txn.commit().await?;

        // 팔로우 삭제 이벤트 로깅
        repository_log_event(
            conn,
            Some(follower.id),
            ActionType::FollowDeleted,
            Some(followee.id),
            Some(TargetType::User),
            None,
        )
        .await;

        Ok(())
    } else {
        Err(Errors::FollowNotExist)
    }
}
