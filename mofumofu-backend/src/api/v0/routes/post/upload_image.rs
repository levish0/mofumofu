use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::post::request::image_upload::ImageUploadForm;
use crate::service::error::errors::Errors;
use crate::service::post::upload_image::service_upload_image;
use crate::state::AppState;
use axum::extract::{Multipart, State};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde::Serialize;
use tracing::info;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ImageUploadResponse {
    pub filename: String,
}

#[utoipa::path(
    post,
    path = "/v0/post/image",
    request_body(content = ImageUploadForm, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Image upload queued successfully", body = ImageUploadResponse),
        (status = 400, description = "Invalid file or parameters"),
        (status = 401, description = "Unauthorized"),
        (status = 413, description = "File too large"),
        (status = 422, description = "Unsupported image format"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Post"
)]
pub async fn upload_image(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    multipart: Multipart,
) -> Result<impl IntoResponse, Errors> {
    info!(
        "Received image upload request by user: {}",
        claims.sub
    );

    let filename = service_upload_image(&state.http_client, &claims.sub.to_string(), multipart).await?;

    Ok(Json(ImageUploadResponse { filename }))
}