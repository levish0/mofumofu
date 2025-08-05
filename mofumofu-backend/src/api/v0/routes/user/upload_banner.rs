use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::user::request::banner_image::ProfileBannerForm;
use crate::service::error::errors::Errors;
use crate::state::AppState;
use axum::Extension;
use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use tracing::info;
use utoipa::ToSchema;
use crate::service::user::service_update_user_banner;

#[utoipa::path(
    post,
    path = "/v0/user/profile/banner",
    request_body(content = ProfileBannerForm, content_type = "multipart/form-data"),
    responses(
        (status = 204, description = "Banner image upload queued successfully"),
        (status = 400, description = "Invalid file or parameters"),
        (status = 401, description = "Unauthorized"),
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

    service_update_user_banner(&state.conn, &state.http_client, &claims.sub, multipart).await?;

    Ok(StatusCode::NO_CONTENT)
}
