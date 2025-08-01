use crate::entity::users::Entity as UsersEntity;
use crate::entity::users::{GetFollowersLink, Model as UsersModel};
use crate::repository::user::get_user_by_handle::repository_get_user_by_handle;
use crate::service::error::errors::Errors;
use sea_orm::ConnectionTrait;
use sea_orm::EntityTrait;
use sea_orm::QuerySelect;
use sea_orm::TransactionTrait;

pub async fn service_get_followers<C>(
    conn: &C,
    user_handle: &str,
    offset: u64,
    limit: u64,
) -> anyhow::Result<Vec<UsersModel>, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user = repository_get_user_by_handle(conn, user_handle).await?;

    let followers_with_user = UsersEntity::find_by_id(user.id)
        .find_also_linked(GetFollowersLink)
        .offset(offset)
        .limit(limit)
        .all(conn)
        .await?;

    // Extract the second element (follower users) from each tuple and filter out None values
    let followers: Vec<UsersModel> = followers_with_user
        .into_iter()
        .filter_map(|(_, follower_user)| follower_user)
        .collect();

    Ok(followers)
}
