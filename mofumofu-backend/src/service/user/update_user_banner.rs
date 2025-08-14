use crate::service::error::errors::{Errors, ServiceResult};
use crate::repository::user::get_user_by_uuid::repository_get_user_by_uuid;
use crate::microservices::profile_client::queue_user_banner_update;
use crate::utils::image_validator::{generate_image_hash, validate_and_get_image_info};
use axum::extract::Multipart;
use reqwest::Client;
use sea_orm::ConnectionTrait;
use tracing::{error, info, warn};
use uuid::Uuid;

pub async fn service_update_user_banner<C>(
    conn: &C,
    http_client: &Client,
    user_uuid: &Uuid,
    mut multipart: Multipart,
) -> ServiceResult<String>
where
    C: ConnectionTrait,
{
    info!("Processing banner image upload for user: {}", user_uuid);

    // UUID로 사용자 정보 조회
    let user = repository_get_user_by_uuid(conn, user_uuid).await?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| Errors::BadRequestError(format!("Failed to read multipart field: {}", e)))?
    {
        if field.name() == Some("file") {
            let data = field
                .bytes()
                .await
                .map_err(|e| Errors::BadRequestError(format!("Failed to read file data: {}", e)))?;

            // Validate image and get info (8MB limit for banner)
            const MAX_BANNER_SIZE: usize = 8 * 1024 * 1024;
            let (content_type, extension) = validate_and_get_image_info(&data, MAX_BANNER_SIZE)?;
            
            // Generate hash-based filename
            let hash = generate_image_hash(&data);
            let filename = format!("banner_{}.{}", hash, extension);

            info!(
                "Processing banner image upload: user_uuid={}, filename={}, content_type={}, size={} bytes",
                user_uuid,
                filename,
                content_type,
                data.len()
            );

            // 태스크 큐에 업데이트 요청 (기존 삭제 후 새 업로드)
            queue_user_banner_update(
                http_client,
                &user_uuid,
                &user.handle,
                &filename,
                data.to_vec(),
                &content_type,
            )
            .await
            .map_err(|e| {
                error!("Failed to queue banner image upload task: {}", e);
                Errors::SysInternalError("Failed to queue banner image upload task".to_string())
            })?;

            info!("Banner image upload task queued for user: {}", user_uuid);
            return Ok(filename);
        }
    }

    Err(Errors::BadRequestError("No file found in request".to_string()))
}
