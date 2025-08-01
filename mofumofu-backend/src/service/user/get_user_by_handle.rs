use crate::dto::user::response::info::UserInfoResponse;
use crate::repository::user::get_user_by_handle::repository_get_user_by_handle;
use crate::service::error::errors::Errors;
use sea_orm::{ConnectionTrait, EntityTrait, TransactionTrait};

pub async fn service_get_user_by_handle<C>(
    conn: &C,
    handle: &str,
) -> Result<UserInfoResponse, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user = repository_get_user_by_handle(conn, handle).await?;

    Ok(UserInfoResponse {
        name: user.name,
        handle: user.handle,
        email: user.email,
        profile_image: user.profile_image,
        banner_image: user.banner_image,
    })
}
