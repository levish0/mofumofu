use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::user::response::info::UserInfoResponse;
use crate::service::error::errors::Errors;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use tracing::info;
use crate::dto::user::request::update_profile::UpdateProfileRequest;
use crate::service::user::service_update_user_profile;
use crate::service::validator::json_validator::ValidatedJson;

#[utoipa::path(
    put,
    path = "/v0/user/profile",
    request_body = UpdateProfileRequest,
    responses(
        (status = StatusCode::OK, description = "Profile updated successfully", body = UserInfoResponse),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input"),
        (status = StatusCode::UNAUTHORIZED, description = "Unauthorized"),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "User"
)]
pub async fn update_profile(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<UpdateProfileRequest>,
) -> Result<UserInfoResponse, Errors> {
    info!("Received PUT request to update profile for user: {}", claims.sub);

    let updated_user = service_update_user_profile(&state.conn, &claims.sub, payload).await?;

    Ok(updated_user)
}