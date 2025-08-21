use crate::{
    microservices::admin_tasks_client::sync_all_counts,
    service::auth::role_check::require_admin,
    service::error::errors::{Errors, ServiceResult},
    state::AppState,
};
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;

pub async fn service_sync_all_counts(
    app_state: &AppState,
    user_id: Uuid,
) -> ServiceResult<serde_json::Value> {
    // Admin 권한 확인
    require_admin(&app_state.conn, user_id).await?;

    info!("Admin user {} triggering all counts sync", user_id);

    match sync_all_counts(&app_state.http_client).await {
        Ok(response) => {
            info!("Successfully triggered all counts sync");
            Ok(json!({
                "status": response.status,
                "message": response.message,
                "updated_count": response.updated_count,
                "follower_updated_count": response.follower_updated_count,
                "following_updated_count": response.following_updated_count,
                "total_updated": response.total_updated
            }))
        }
        Err(e) => {
            error!("Failed to sync all counts: {}", e);
            Err(Errors::SysInternalError(e.to_string()))
        }
    }
}