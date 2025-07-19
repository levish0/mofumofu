use crate::dto::user::internal::info::UserInfo;
use crate::dto::user::response::info::UserInfoResponse;
use crate::entity::users::{Column, Entity as UserEntity};
use crate::service::error::errors::Errors;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    QueryFilter, TransactionTrait,
};
use tracing::error;

pub async fn service_get_user_by_handle(
    conn: &DatabaseConnection,
    handle: &str,
) -> anyhow::Result<UserInfoResponse, Errors> {
    let user = UserEntity::find()
        .filter(Column::Handle.eq(handle))
        .one(conn)
        .await?;

    match user {
        Some(user) => Ok(UserInfoResponse {
            name: user.name,
            handle: user.handle,
            email: user.email,
        }),
        None => {
            error!("User not found with handle: {}", handle);
            Err(Errors::UserNotFound)
        }
    }
}

pub async fn get_user_by_handle<C>(conn: &C, handle: &str) -> anyhow::Result<UserInfo, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user = UserEntity::find()
        .filter(Column::Handle.eq(handle))
        .one(conn)
        .await?;

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
            error!("User not found with handle: {}", handle);
            Err(Errors::UserNotFound)
        }
    }
}
