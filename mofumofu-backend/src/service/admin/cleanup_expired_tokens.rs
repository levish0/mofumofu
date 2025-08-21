use crate::{
    microservices::admin_tasks_client::cleanup_expired_refresh_tokens,
    service::auth::role_check::require_admin,
    service::error::errors::{Errors, ServiceResult},
    state::AppState,
};
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;

pub async fn service_cleanup_expired_tokens(
    app_state: &AppState,
    user_id: Uuid,
) -> ServiceResult<serde_json::Value> {
    // Admin 권한 확인
    require_admin(&app_state.conn, user_id).await?;

    info!("Admin user {} triggering expired tokens cleanup", user_id);

    match cleanup_expired_refresh_tokens(&app_state.http_client).await {
        Ok(response) => {
            info!("Successfully triggered expired tokens cleanup");
            Ok(json!({
                "status": response.status,
                "message": response.message,
                "expired_tokens_deleted": response.expired_tokens_deleted,
                "revoked_tokens_deleted": response.revoked_tokens_deleted,
                "total_deleted": response.total_deleted
            }))
        }
        Err(e) => {
            error!("Failed to cleanup expired tokens: {}", e);
            Err(Errors::SysInternalError(e.to_string()))
        }
    }
}