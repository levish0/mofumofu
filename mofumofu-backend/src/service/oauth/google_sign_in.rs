use reqwest::Client;
use crate::dto::auth::response::jwt::AuthJWTResponse;
use crate::entity::common::OAuthProvider;
use crate::entity::user_refresh_tokens::ActiveModel as RefreshTokenActiveModel;
use crate::service::auth::jwt::{create_jwt_access_token, create_jwt_refresh_token};
use crate::service::error::errors::Errors;
use crate::service::oauth::find_or_create_oauth_user::service_find_or_create_oauth_user;
use crate::service::oauth::provider::common::exchange_oauth_code;
use crate::service::oauth::provider::google::client::{exchange_google_code, get_google_user_info};
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set, TransactionTrait};
use tracing::{error, info, warn};
use crate::connection::cloudflare_r2::R2Client;
use crate::dto::user::internal::update_user::UpdateUserFields;
use crate::repository::user::update_user::repository_update_user;
use crate::utils::save_user_profile_to_r2::save_user_profile_to_r2;

pub async fn service_google_sign_in<C>(
    txn: &C,
    http_client: &Client,
    r2_client: &R2Client,
    user_agent: Option<String>,
    ip_address: Option<String>,
    auth_code: &str,
) -> Result<AuthJWTResponse, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    // 1. 구글에서 액세스 토큰 획득
    let access_token = exchange_google_code(auth_code).await?;
    // 2. 구글에서 유저 정보 획득
    let google_user = get_google_user_info(http_client, r2_client, &access_token).await?;

    // 3. 유저 찾기 또는 생성
    let user = service_find_or_create_oauth_user(
        txn,
        &google_user.email,
        &google_user.name,
        &google_user.sub,
        OAuthProvider::Google,
        google_user.picture.clone(),
    )
    .await?;

    // 4. 프로필 이미지 처리 및 데이터베이스 업데이트
    if let Some(google_picture_url) = &google_user.picture {
        match save_user_profile_to_r2(http_client, r2_client, &user.handle, google_picture_url).await {
            Ok(r2_url) => {
                info!("Profile image saved to R2 for user {}: {}", user.id, r2_url);

                let update_fields = UpdateUserFields {
                    profile_image: Some(Some(r2_url.clone())),
                    ..Default::default()
                };
                repository_update_user(txn, &user.id, update_fields).await?;
            }
            Err(e) => {
                warn!("Failed to save profile image to R2 for user {}: {:?}", user.id, e);
            }
        }
    }

    // 5. JWT 토큰 생성
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

    refresh_model.insert(txn).await.map_err(|e| {
        error!("Failed to save refresh token: {:?}", e);
        Errors::DatabaseError(e.to_string())
    })?;

    info!(
        "Successfully logged in user via Google OAuth: {}",
        user.email
    );

    Ok(AuthJWTResponse {
        access_token,
        cookie_refresh_token: refresh_token.token,
    })
}
