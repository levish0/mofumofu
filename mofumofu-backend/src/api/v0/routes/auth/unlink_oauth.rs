use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::auth::request::unlink_oauth::UnlinkOAuthRequest;
use crate::service::auth::service_unlink_oauth;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use tracing::info;

#[utoipa::path(
    delete,
    path = "/v0/auth/unlink-oauth",
    request_body = UnlinkOAuthRequest,
    responses(
        (status = 200, description = "OAuth connection removed successfully"),
        (status = 400, description = "Cannot unlink last OAuth connection or connection not found"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Auth",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn unlink_oauth(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<UnlinkOAuthRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received DELETE request to unlink OAuth {:?} for user: {}", payload.provider, claims.sub);

    service_unlink_oauth(&state.conn, &claims.sub, payload).await?;

    Ok(StatusCode::OK)
}