use crate::dto::auth::request::login::AuthLoginRequest;
use crate::dto::auth::response::jwt::AuthJWTResponse;
use crate::service::auth::auth::service_sign_in;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use crate::utils::extract_ip_address::extract_ip_address;
use crate::utils::extract_user_agent::extract_user_agent;
use axum::extract::{ConnectInfo, State};
use axum::http::HeaderMap;
use axum_extra::TypedHeader;
use axum_extra::headers::UserAgent;
use std::net::SocketAddr;

#[utoipa::path(
    post,
    path = "/v0/auth/sign_in",
    request_body = AuthLoginRequest,
    responses(
        (status = StatusCode::OK, description = "Login successful", body = AuthJWTResponse),
        (status = StatusCode::UNAUTHORIZED, description = "Invalid credentials"),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::BAD_REQUEST, description = "Invalid request"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    tag = "Auth"
)]
pub async fn sign_in(
    user_agent: Option<TypedHeader<UserAgent>>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<AuthLoginRequest>,
) -> Result<AuthJWTResponse, Errors> {
    let ip_str = extract_ip_address(&headers, addr);
    let ua_str = extract_user_agent(user_agent);

    let res = service_sign_in(&state.conn, Some(ua_str), Some(ip_str), payload).await?;

    Ok(AuthJWTResponse {
        access_token: res.access_token,
        cookie_refresh_token: res.cookie_refresh_token,
    })
}
