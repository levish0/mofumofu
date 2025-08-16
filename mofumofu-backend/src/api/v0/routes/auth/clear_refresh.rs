use crate::dto::auth::internal::refresh_token::RefreshTokenContext;
use crate::dto::auth::response::sign_out::SignOutResponse;
use crate::service::auth::service_sign_out;
use crate::service::error::errors::Errors;
use crate::state::AppState;
use crate::utils::extract_ip_address::extract_ip_address;
use crate::utils::extract_user_agent::extract_user_agent;
use axum::extract::{ConnectInfo, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Extension;
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use std::net::SocketAddr;

#[utoipa::path(
    post,
    path = "/v0/auth/clear_refresh",
    responses(
        (status = 204, description = "Refresh token cookie deleted successfully."),
    ),
    tag = "Auth"
)]
pub async fn clear_refresh(
) -> Result<impl IntoResponse, Errors> {
    Ok(SignOutResponse)
}
