use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::post::request::thumbnail_image::PostThumbnailForm;
use crate::service::error::errors::Errors;
use crate::service::post::update_post_thumbnail::service_update_post_thumbnail;
use crate::state::AppState;
use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/post/thumbnail",
    request_body(content = PostThumbnailForm, content_type = "multipart/form-data"),
    responses(
        (status = 204, description = "Thumbnail image upload queued successfully"),
        (status = 400, description = "Invalid file or parameters"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Not the owner of the post"),
        (status = 404, description = "Post not found"),
        (status = 413, description = "File too large"),
        (status = 422, description = "Unsupported image format"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Post"
)]
pub async fn upload_thumbnail(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    multipart: Multipart,
) -> Result<impl IntoResponse, Errors> {
    info!(
        "Received thumbnail image upload request by user: {}",
        claims.sub
    );

    service_update_post_thumbnail(&state.conn, &state.http_client, &claims.sub, multipart).await?;

    Ok(StatusCode::NO_CONTENT)
}
