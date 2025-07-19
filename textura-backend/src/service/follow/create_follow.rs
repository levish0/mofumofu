use crate::dto::follow::internal::create::CreateFollow;
use crate::entity::follows::ActiveModel as FollowsActiveModel;
use crate::entity::follows::Column as FollowsColumn;
use crate::entity::follows::Entity as FollowsEntity;
use crate::service::error::errors::Errors;
use crate::service::user::{get_user_by_handle, get_user_by_uuid};
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

    let follower = get_user_by_uuid(&txn, &payload.follower_id).await?;
    let followee = get_user_by_handle(&txn, &payload.followee_handle).await?;

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
    new_follow.insert(&txn).await?;

    // Commit the transaction
    txn.commit().await?;

    Ok(())
}
