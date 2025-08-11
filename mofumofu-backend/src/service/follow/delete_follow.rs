use crate::dto::follow::internal::delete::DeleteFollow;
use crate::entity::common::{ActionType, TargetType};
use crate::repository::system_events::log_event::repository_log_event;
use crate::repository::user::find_user_by_handle::repository_find_user_by_handle;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::Errors;
use sea_orm::TransactionTrait;
use sea_orm::{ActiveModelTrait, ConnectionTrait, QueryFilter};
use sea_orm::{ColumnTrait, EntityTrait};

pub async fn service_delete_follow_by_handle<C>(
    conn: &C,
    payload: DeleteFollow,
) -> anyhow::Result<(), Errors>
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

    let existing_follow = crate::entity::follows::Entity::find()
        .filter(crate::entity::follows::Column::FollowerId.eq(follower.id))
        .filter(crate::entity::follows::Column::FolloweeId.eq(followee.id))
        .one(&txn)
        .await?;

    match existing_follow {
        Some(follow_record) => {
            let follow_active_model: crate::entity::follows::ActiveModel = follow_record.into();
            follow_active_model.delete(&txn).await?;
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
        }
        None => Err(Errors::FollowNotExist),
    }
}
