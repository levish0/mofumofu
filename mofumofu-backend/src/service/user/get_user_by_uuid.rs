use crate::dto::user::response::info::UserInfoResponse;
use crate::repository::user::get_user_by_uuid::repository_get_user_by_uuid;
use crate::service::error::errors::Errors;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_get_user_by_uuid(
    conn: &DatabaseConnection,
    user_uuid: &Uuid,
) -> Result<UserInfoResponse, Errors> {
    let user = repository_get_user_by_uuid(conn, user_uuid).await?;

    Ok(UserInfoResponse {
        name: user.name,
        handle: user.handle,
        email: user.email,
        profile_image: user.profile_image,
        banner_image: user.banner_image,
    })
}
