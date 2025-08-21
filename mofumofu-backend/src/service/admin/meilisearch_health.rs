use crate::{
    microservices::admin_tasks_client::check_meilisearch_health,
    service::auth::role_check::require_admin,
    service::error::errors::{Errors, ServiceResult},
    state::AppState,
};
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;

pub async fn service_meilisearch_health(
    app_state: &AppState,
    user_id: Uuid,
) -> ServiceResult<serde_json::Value> {
    // Admin 권한 확인
    require_admin(&app_state.conn, user_id).await?;

    info!("Admin user {} checking Meilisearch health", user_id);

    match check_meilisearch_health(&app_state.http_client).await {
        Ok(response) => {
            info!("Successfully checked Meilisearch health");
            Ok(json!({
                "status": response.status,
                "message": response.message,
                "health": response.health
            }))
        }
        Err(e) => {
            error!("Failed to check Meilisearch health: {}", e);
            Err(Errors::SysInternalError(e.to_string()))
        }
    }
}