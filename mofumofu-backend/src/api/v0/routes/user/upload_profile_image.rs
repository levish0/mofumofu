use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::service::error::errors::Errors;

use crate::state::AppState;
use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use serde::Deserialize;
use tracing::info;
use utoipa::ToSchema;
use crate::service::user::upload_user_banner::service_upload_user_profile;

/// Schema for profile image upload multipart form
#[derive(Deserialize, ToSchema)]
#[allow(unused)]
pub struct ProfileImageForm {
    #[schema(format = Binary, content_media_type = "image/*")]
    file: String,
}

#[utoipa::path(
    post,
    path = "/v0/user/profile/image",
    request_body(content = ProfileImageForm, content_type = "multipart/form-data"),
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
pub async fn upload_profile_image(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    multipart: Multipart,
) -> Result<impl IntoResponse, Errors> {
    info!("Received profile image upload request for user: {}", claims.sub);

    service_upload_user_profile(&state.conn, &state.http_client, &claims.sub, multipart, "profile").await?;

    Ok(StatusCode::NO_CONTENT)
}