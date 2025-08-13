use crate::repository::user::get_user_by_uuid::repository_get_user_by_uuid;
use crate::service::error::errors::Errors;
use crate::microservices::profile_client::queue_user_avatar_delete;
use reqwest::Client;
use sea_orm::ConnectionTrait;
use tracing::{error, info};
use uuid::Uuid;

pub async fn service_delete_user_avatar<C>(
    conn: &C,
    http_client: &Client,
    user_uuid: &Uuid,
) -> Result<(), Errors>
where
    C: ConnectionTrait,
{
    info!("Processing avatar image delete for user: {}", user_uuid);

    // UUID로 사용자 정보 조회
    let user = repository_get_user_by_uuid(conn, user_uuid).await?;

    // 태스크 큐에 삭제 요청
    queue_user_avatar_delete(http_client, &user_uuid, &user.handle)
        .await
        .map_err(|e| {
            error!("Failed to queue avatar image delete task: {}", e);
            Errors::SysInternalError("Failed to queue avatar image delete task".to_string())
        })?;

    info!("Avatar image delete task queued for user: {}", user_uuid);

    Ok(())
}
