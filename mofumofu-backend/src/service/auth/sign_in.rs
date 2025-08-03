use crate::dto::auth::request::login::AuthLoginRequest;
use crate::dto::auth::response::jwt::AuthJWTResponse;
use crate::entity::user_refresh_tokens::ActiveModel as RefreshTokenActiveModel;
use crate::repository::auth::create_refresh_token::repository_create_refresh_token;
use crate::repository::user::find_user_by_handle::repository_find_user_by_handle;
use crate::service::auth::jwt::{create_jwt_access_token, create_jwt_refresh_token};
use crate::service::error::errors::Errors;
use crate::utils::crypto::verify_password;
use anyhow::Result;
use sea_orm::{ConnectionTrait, Set, TransactionTrait};
use tracing::error;

pub async fn service_sign_in<C>(
    conn: &C,
    user_agent: Option<String>,
    ip_address: Option<String>,
    payload: AuthLoginRequest,
) -> Result<AuthJWTResponse, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user = repository_find_user_by_handle(conn, &payload.handle)
        .await?
        .ok_or_else(|| {
            error!("User not found with handle: {}", payload.handle);
            Errors::UserNotFound
        })?;

    verify_password(&payload.password, &user.password)?;

    let access_token = create_jwt_access_token(&user.id).map_err(|e| {
        error!("Failed to create access token: {:?}", e);
        Errors::TokenCreationError(e.to_string())
    })?;

    let refresh_token = create_jwt_refresh_token(&user.id).map_err(|e| {
        error!("Failed to create refresh token: {:?}", e);
        Errors::TokenCreationError(e.to_string())
    })?;

    let refresh_model = RefreshTokenActiveModel {
        id: Set(refresh_token.jti),
        user_id: Set(user.id),
        ip_address: Set(ip_address),
        user_agent: Set(user_agent),
        refresh_token: Set(refresh_token.token.clone()),
        expires_at: Set(refresh_token.expires_at),
        created_at: Set(refresh_token.issued_at),
        revoked_at: Default::default(),
    };

    repository_create_refresh_token(conn, refresh_model)
        .await
        .map(|_| AuthJWTResponse {
            access_token,
            cookie_refresh_token: refresh_token.token,
        })
        .map_err(|e| {
            error!("Failed to login user: {:?}", e);
            e
        })
}
