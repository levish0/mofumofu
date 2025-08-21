use crate::{
    microservices::admin_tasks_client::sync_post_like_counts,
    service::auth::role_check::require_admin,
    service::error::errors::{Errors, ServiceResult},
    state::AppState,
};
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;

pub async fn service_sync_likes(
    app_state: &AppState,
    user_id: Uuid,
) -> ServiceResult<serde_json::Value> {
    // Admin 권한 확인
    require_admin(&app_state.conn, user_id).await?;

    info!("Admin user {} triggering like counts sync", user_id);

    match sync_post_like_counts(&app_state.http_client).await {
        Ok(response) => {
            info!("Successfully triggered like counts sync");
            Ok(json!({
                "status": response.status,
                "message": response.message,
                "updated_count": response.updated_count
            }))
        }
        Err(e) => {
            error!("Failed to sync like counts: {}", e);
            Err(Errors::SysInternalError(e.to_string()))
        }
    }
}