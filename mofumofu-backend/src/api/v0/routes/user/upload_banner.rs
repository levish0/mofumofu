use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::user::request::banner_image::ProfileBannerForm;
use crate::dto::user::response::image_upload::ImageUploadResponse;
use crate::service::error::errors::Errors;
use crate::service::user::update_user_banner::service_update_user_banner;
use crate::state::AppState;
use axum::extract::{Multipart, State};
use axum::response::IntoResponse;
use axum::Extension;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/user/profile/banner",
    request_body(content = ProfileBannerForm, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Banner image upload queued successfully", body = ImageUploadResponse),
        (status = 400, description = "Invalid file or parameters"),
        (status = 401, description = "Unauthorized"),
        (status = 413, description = "File too large"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "User"
)]
pub async fn upload_banner(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    multipart: Multipart,
) -> Result<impl IntoResponse, Errors> {
    info!(
        "Received banner image upload request for user: {}",
        claims.sub
    );

    let filename = service_update_user_banner(&state.conn, &state.cloudflare_r2, &claims.sub, multipart).await?;

    Ok(ImageUploadResponse { filename })
}
