use crate::dto::auth::response::jwt::AuthJWTResponse;
use crate::entity::common::OAuthProvider;
use crate::entity::user_refresh_tokens::ActiveModel as RefreshTokenActiveModel;
use crate::service::auth::jwt::{create_jwt_access_token, create_jwt_refresh_token};
use crate::service::error::errors::Errors;
use crate::service::oauth::find_or_create_oauth_user::service_find_or_create_oauth_user;
use crate::service::oauth::provider::common::exchange_oauth_code;
use crate::service::oauth::provider::github::client::{exchange_github_code, get_github_user_info};
use crate::service::oauth::provider::google::client::{exchange_google_code, get_google_user_info};
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set, TransactionTrait};
use tracing::{error, info};

pub async fn service_github_sign_in<C>(
    txn: &C,
    user_agent: Option<String>,
    ip_address: Option<String>,
    auth_code: &str,
) -> Result<AuthJWTResponse, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    // 1. GitHub에서 액세스 토큰 획득
    let access_token = exchange_github_code(auth_code).await?;

    // 2. GitHub에서 유저 정보 획득
    let github_user = get_github_user_info(&access_token).await?;
    // 3. 이메일이 없으면 에러
    let email = github_user.email.ok_or_else(|| {
        error!("GitHub user has no email address");
        Errors::OauthUserInfoParseFailed
    })?;

    // 4. 유저 찾기 또는 생성
    let user = service_find_or_create_oauth_user(
        txn,
        &email,
        &github_user.name.unwrap_or(github_user.login.clone()),
        &github_user.id.to_string(),
        OAuthProvider::Github,
        Some(github_user.avatar_url),
    )
    .await?;
    // 5. JWT 토큰 생성 (Google과 동일한 로직)
    let access_token = create_jwt_access_token(&user.id).map_err(|e| {
        error!("Failed to create access token: {:?}", e);
        Errors::TokenCreationError(e.to_string())
    })?;

    let refresh_token = create_jwt_refresh_token(&user.id).map_err(|e| {
        error!("Failed to create refresh token: {:?}", e);
        Errors::TokenCreationError(e.to_string())
    })?;

    // 6. 리프레시 토큰 DB에 저장
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

    refresh_model.insert(txn).await.map_err(|e| {
        error!("Failed to save refresh token: {:?}", e);
        Errors::DatabaseError(e.to_string())
    })?;

    info!(
        "Successfully logged in user via GitHub OAuth: {}",
        user.email
    );

    Ok(AuthJWTResponse {
        access_token,
        cookie_refresh_token: refresh_token.token,
    })
}
