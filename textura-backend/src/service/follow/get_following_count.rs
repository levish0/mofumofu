use crate::service::error::errors::Errors;
use crate::service::user::get_user_by_handle;
use sea_orm::PaginatorTrait;
use sea_orm::TransactionTrait;
use sea_orm::{ConnectionTrait, QueryFilter};
use sea_orm::{ColumnTrait, EntityTrait};

pub async fn service_get_following_count<C>(
    conn: &C,
    user_handle: &str,
) -> anyhow::Result<u64, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user = get_user_by_handle(conn, user_handle).await?;

    let count = crate::entity::follows::Entity::find()
        .filter(crate::entity::follows::Column::FollowerId.eq(user.id))
        .count(conn)
        .await?;

    Ok(count)
}
