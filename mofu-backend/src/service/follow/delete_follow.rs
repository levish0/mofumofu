use crate::dto::follow::internal::delete::DeleteFollow;
use crate::service::error::errors::Errors;
use crate::service::user::{get_user_by_handle, get_user_by_uuid};
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

    let follower = get_user_by_uuid(&txn, &payload.follower_id).await?;
    let followee = get_user_by_handle(&txn, &payload.followee_handle).await?;

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
            Ok(())
        }
        None => Err(Errors::FollowNotExist),
    }
}
