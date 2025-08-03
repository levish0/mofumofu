use crate::dto::user::request::update_profile::UpdateProfileRequest;
use crate::dto::user::response::info::UserInfoResponse;

use crate::dto::user::internal::update_user::UpdateUserFields;
use crate::repository::user::update_user::repository_update_user;
use crate::service::error::errors::Errors;
use sea_orm::{ConnectionTrait, TransactionTrait};
use uuid::Uuid;

pub async fn service_update_user_profile<C>(
    conn: &C,
    user_uuid: &Uuid,
    payload: UpdateProfileRequest,
) -> Result<UserInfoResponse, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    // payload에서 업데이트할 필드만 추출
    let update_fields = UpdateUserFields {
        name: payload.name,
        handle: payload.handle,
        ..Default::default() // 나머지 필드는 None
    };

    let updated_user = repository_update_user(&txn, user_uuid, update_fields).await?;

    txn.commit().await?;

    Ok(UserInfoResponse {
        name: updated_user.name,
        handle: updated_user.handle,
        email: updated_user.email,
        profile_image: updated_user.profile_image,
        banner_image: updated_user.banner_image,
    })
}
