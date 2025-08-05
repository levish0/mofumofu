use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::service::error::errors::Errors;

use crate::dto::user::request::avatar_image::ProfileAvatarForm;
use crate::service::user::update_user_avatar::{service_update_user_avatar};
use crate::state::AppState;
use axum::Extension;
use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use tracing::info;
use utoipa::ToSchema;

#[utoipa::path(
    post,
    path = "/v0/user/profile/avatar",
    request_body(content = ProfileAvatarForm, content_type = "multipart/form-data"),
    responses(
        (status = 204, description = "Profile image upload queued successfully"),
        (status = 400, description = "Invalid file or parameters"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "User"
)]
pub async fn upload_avatar(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    multipart: Multipart,
) -> Result<impl IntoResponse, Errors> {
    info!(
        "Received profile image upload request for user: {}",
        claims.sub
    );

    service_update_user_avatar(&state.conn, &state.http_client, &claims.sub, multipart).await?;

    Ok(StatusCode::NO_CONTENT)
}
