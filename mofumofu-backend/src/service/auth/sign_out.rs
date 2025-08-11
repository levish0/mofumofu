use crate::dto::auth::internal::refresh_token::RefreshTokenClaims;
use crate::entity::common::{ActionType, TargetType};
use crate::repository::auth::find_refresh_token_by_jti_and_token::repository_find_refresh_token_by_jti_and_token;
use crate::repository::auth::revoke_refresh_token::repository_revoke_refresh_token;
use crate::repository::system_events::log_event::repository_log_event;
use crate::service::error::errors::Errors;
use chrono::Utc;
use sea_orm::{ConnectionTrait, TransactionTrait};
use tracing::error;

pub async fn service_sign_out<C>(
    conn: &C,
    user_agent: Option<String>,
    ip_address: Option<String>,
    refresh_token: String,
    refresh_token_claims: RefreshTokenClaims,
) -> Result<(), Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let now = Utc::now().timestamp();
    if refresh_token_claims.exp < now {
        error!(
            "Refresh token has expired: token_exp={}, now={}",
            refresh_token_claims.exp, now
        );
        return Err(Errors::UserTokenExpired);
    }

    let stored_token = repository_find_refresh_token_by_jti_and_token(
        conn,
        refresh_token_claims.jti,
        refresh_token,
    )
    .await?
    .ok_or_else(|| {
        error!("Refresh token not found or already revoked");
        Errors::UserInvalidToken
    })?;

    let result = repository_revoke_refresh_token(conn, stored_token, ip_address, user_agent, Utc::now())
        .await
        .map(|_| ())
        .map_err(|e| {
            error!("Failed to revoke refresh token: {:?}", e);
            e
        });

    // 로그아웃 성공 시 이벤트 로깅
    if result.is_ok() {
        repository_log_event(
            conn,
            Some(refresh_token_claims.sub),
            ActionType::UserSignedOut,
            Some(refresh_token_claims.sub),
            Some(TargetType::User),
            None,
        )
        .await;
    }

    result
}
