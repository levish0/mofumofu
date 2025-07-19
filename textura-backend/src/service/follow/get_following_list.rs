use crate::dto::follow::internal::delete::DeleteFollow;
use crate::entity::follows::ActiveModel as FollowsActiveModel;
use crate::entity::follows::Column as FollowsColumn;
use crate::entity::follows::Entity as FollowsEntity;
use crate::entity::users::Entity as UsersEntity;
use crate::entity::users::Relation as UsersRelation;
use crate::entity::users::{GetFollowersLink, GetFollowingLink, Model as UsersModel};
use crate::service::error::errors::Errors;
use crate::service::user::{get_user_by_handle, get_user_by_uuid};
use sea_orm::{ActiveModelTrait, ConnectionTrait, QueryFilter, Set};
use sea_orm::{ColumnTrait, EntityTrait};
use sea_orm::{JoinType, TransactionTrait};
use sea_orm::{PaginatorTrait, QuerySelect};

pub async fn service_get_following<C>(
    conn: &C,
    user_handle: &str,
    offset: u64,
    limit: u64,
) -> anyhow::Result<Vec<UsersModel>, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user = get_user_by_handle(conn, user_handle).await?;

    // Join을 사용한 더 효율적인 쿼리
    let following_with_user = UsersEntity::find_by_id(user.id)
        .find_also_linked(GetFollowingLink)
        .offset(offset)
        .limit(limit)
        .all(conn)
        .await?;

    // Extract the second element (following users) from each tuple and filter out None values
    let following: Vec<UsersModel> = following_with_user
        .into_iter()
        .filter_map(|(_, following_user)| following_user)
        .collect();

    Ok(following)
}
