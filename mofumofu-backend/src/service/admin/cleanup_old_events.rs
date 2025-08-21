use crate::{
    microservices::admin_tasks_client::cleanup_old_system_events,
    service::auth::role_check::require_admin,
    service::error::errors::{Errors, ServiceResult},
    state::AppState,
};
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;

pub async fn service_cleanup_old_events(
    app_state: &AppState,
    user_id: Uuid,
) -> ServiceResult<serde_json::Value> {
    // Admin 권한 확인
    require_admin(&app_state.conn, user_id).await?;

    info!("Admin user {} triggering old events cleanup", user_id);

    match cleanup_old_system_events(&app_state.http_client).await {
        Ok(response) => {
            info!("Successfully triggered old events cleanup");
            Ok(json!({
                "status": response.status,
                "message": response.message,
                "deleted_count": response.deleted_count
            }))
        }
        Err(e) => {
            error!("Failed to cleanup old events: {}", e);
            Err(Errors::SysInternalError(e.to_string()))
        }
    }
}