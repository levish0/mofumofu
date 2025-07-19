use crate::dto::user::internal::info::UserInfo;
use crate::dto::user::response::info::UserInfoResponse;
use crate::entity::users::Entity as UserEntity;
use crate::service::error::errors::Errors;
use sea_orm::{
    ConnectionTrait, DatabaseConnection, EntityTrait, TransactionTrait,
};
use tracing::error;
use uuid::Uuid;

pub async fn service_get_user_by_uuid(
    conn: &DatabaseConnection,
    user_uuid: &Uuid,
) -> anyhow::Result<UserInfoResponse, Errors> {
    let user = UserEntity::find_by_id(*user_uuid).one(conn).await?;

    match user {
        Some(user) => Ok(UserInfoResponse {
            name: user.name,
            handle: user.handle,
            email: user.email,
        }),
        None => {
            error!("User not found with id: {}", user_uuid);
            Err(Errors::UserNotFound)
        }
    }
}

pub async fn get_user_by_uuid<C>(conn: &C, user_uuid: &Uuid) -> anyhow::Result<UserInfo, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user = UserEntity::find_by_id(*user_uuid).one(conn).await?;

    match user {
        Some(user) => Ok(UserInfo {
            id: user.id,
            name: user.name,
            handle: user.handle,
            email: user.email,
            profile_image: user.profile_image,
            banner_image: user.banner_image,
        }),
        None => {
            error!("User not found with id: {}", user_uuid);
            Err(Errors::UserNotFound)
        }
    }
}
