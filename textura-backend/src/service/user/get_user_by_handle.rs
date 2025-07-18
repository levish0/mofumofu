use crate::dto::user::response::info::UserInfoResponse;
use crate::entity::users::{Column, Entity as UserEntity};
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
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
