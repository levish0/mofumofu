use crate::dto::follow::internal::delete::DeleteFollow;
use crate::entity::follows::ActiveModel as FollowsActiveModel;
use crate::entity::follows::Column as FollowsColumn;
use crate::entity::follows::Entity as FollowsEntity;
use crate::entity::users::Entity as UsersEntity;
use crate::entity::users::Relation as UsersRelation;
use crate::entity::users::{GetFollowersLink, Model as UsersModel};
use crate::service::error::errors::Errors;
use crate::service::user::{get_user_by_handle, get_user_by_uuid};
use sea_orm::{ActiveModelTrait, ConnectionTrait, QueryFilter, Set};
use sea_orm::{ColumnTrait, EntityTrait};
use sea_orm::{JoinType, TransactionTrait};
use sea_orm::{PaginatorTrait, QuerySelect};

pub async fn service_get_followers<C>(
    conn: &C,
    user_handle: &str,
    offset: u64,
    limit: u64,
) -> anyhow::Result<Vec<UsersModel>, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user = get_user_by_handle(conn, user_handle).await?;

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
