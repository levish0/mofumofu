use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::auth::internal::refresh_token::RefreshTokenContext;
use crate::dto::user::response::info::UserInfoResponse;
use crate::service::auth::auth::service_sign_out;
use crate::service::error::errors::Errors;
use crate::service::user::service_get_user_by_uuid;
use crate::state::AppState;
use crate::utils::extract_ip_address::extract_ip_address;
use crate::utils::extract_user_agent::extract_user_agent;
use axum::Extension;
use axum::extract::{ConnectInfo, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum_extra::TypedHeader;
use axum_extra::headers::UserAgent;
use std::net::SocketAddr;

#[utoipa::path(
    post,
    path = "/v0/auth/sign_out",
    responses(
        (status = StatusCode::NO_CONTENT, description = "Sign out Successful"),
        (status = StatusCode::UNAUTHORIZED, description = "Refresh token cookie exists but is invalid or malformed"),
        (status = StatusCode::BAD_REQUEST, description = "No refresh token cookie found"),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("refresh_token_cookie" = [])
    ),
    tag = "Auth"
)]
pub async fn sign_out(
    user_agent: Option<TypedHeader<UserAgent>>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Extension(ctx): Extension<RefreshTokenContext>,
) -> Result<impl IntoResponse, Errors> {
    let ip_str = extract_ip_address(&headers, addr);
    let ua_str = extract_user_agent(user_agent);

    let refresh_token = ctx.token;
    let refresh_token_claims = ctx.claims;

    let user = service_sign_out(
        &state.conn,
        Some(ua_str),
        Some(ip_str),
        refresh_token,
        refresh_token_claims,
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
