use crate::repository::user::get_user_by_handle::repository_get_user_by_handle;
use crate::service::error::errors::Errors;
use sea_orm::PaginatorTrait;
use sea_orm::TransactionTrait;
use sea_orm::{ColumnTrait, EntityTrait};
use sea_orm::{ConnectionTrait, QueryFilter};

pub async fn service_get_following_count<C>(
    conn: &C,
    user_handle: &str,
) -> anyhow::Result<u64, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user = repository_get_user_by_handle(conn, user_handle).await?;

    let count = crate::entity::follows::Entity::find()
        .filter(crate::entity::follows::Column::FollowerId.eq(user.id))
        .count(conn)
        .await?;

    Ok(count)
}
