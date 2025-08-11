use crate::dto::follow::internal::create::CreateFollow;
use crate::entity::common::{ActionType, TargetType};
use crate::entity::follows::ActiveModel as FollowsActiveModel;
use crate::entity::follows::Column as FollowsColumn;
use crate::entity::follows::Entity as FollowsEntity;
use crate::repository::system_events::log_event::repository_log_event;
use crate::repository::user::find_user_by_handle::repository_find_user_by_handle;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::Errors;
use sea_orm::TransactionTrait;
use sea_orm::{ActiveModelTrait, ConnectionTrait, QueryFilter, Set};
use sea_orm::{ColumnTrait, EntityTrait};

pub async fn service_create_follow_by_handle<C>(
    conn: &C,
    payload: CreateFollow,
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

    if follower.handle == followee.handle {
        return Err(Errors::FollowCannotFollowSelf);
    }

    // 이미 팔로우 관계가 있는지 체크
    let existing_follow = FollowsEntity::find()
        .filter(FollowsColumn::FollowerId.eq(follower.id))
        .filter(FollowsColumn::FolloweeId.eq(followee.id))
        .one(&txn)
        .await?;

    if existing_follow.is_some() {
        return Err(Errors::FollowAlreadyFollowing);
    }

    let new_follow = FollowsActiveModel {
        id: Default::default(),
        follower_id: Set(follower.id),
        followee_id: Set(followee.id),
    };

    // Insert the new follow relationship
    let created_follow = new_follow.insert(&txn).await?;

    // Commit the transaction
    txn.commit().await?;

    // 팔로우 생성 이벤트 로깅
    repository_log_event(
        conn,
        Some(follower.id),
        ActionType::FollowCreated,
        Some(followee.id),
        Some(TargetType::User),
        None,
    )
    .await;

    Ok(())
}
