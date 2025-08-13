use crate::repository::post::get_post_by_user_and_slug::repository_get_post_by_user_and_slug;
use crate::service::error::errors::Errors;
use crate::microservices::post_client::queue_post_thumbnail_update;
use axum::extract::Multipart;
use chrono::Utc;
use reqwest::Client;
use sea_orm::ConnectionTrait;
use tracing::{error, info, warn};
use uuid::Uuid;

pub async fn service_update_post_thumbnail<C>(
    conn: &C,
    http_client: &Client,
    user_uuid: &Uuid,
    mut multipart: Multipart,
) -> Result<(), Errors>
where
    C: ConnectionTrait,
{
    info!("Processing thumbnail image upload by user: {}", user_uuid);

    let mut file_data: Option<Vec<u8>> = None;
    let mut content_type: Option<String> = None;
    let mut slug: Option<String> = None;

    // multipart 데이터 파싱
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        error!("Failed to read multipart field: {}", e);
        Errors::BadRequestError("Invalid multipart data".to_string())
    })? {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "file" => {
                content_type = field.content_type().map(|ct| ct.to_string());
                let data = field.bytes().await.map_err(|e| {
                    error!("Failed to read image data: {}", e);
                    Errors::BadRequestError("Failed to read image data".to_string())
                })?;

                file_data = Some(data.to_vec());
            }
            "slug" => {
                let text = field.text().await.map_err(|e| {
                    error!("Failed to read slug field: {}", e);
                    Errors::BadRequestError("Failed to read slug".to_string())
                })?;
                slug = Some(text);
            }
            _ => {
                warn!("Unknown field in multipart: {}", field_name);
            }
        }
    }

    // slug 필드 검증 (간단한 길이 체크)
    let slug = slug.ok_or_else(|| {
        error!("No slug provided in multipart data");
        Errors::BadRequestError("Slug is required".to_string())
    })?;

    if slug.trim().is_empty() || slug.len() > 80 {
        return Err(Errors::ValidationError(
            "Slug must be between 1 and 80 characters".to_string(),
        ));
    }

    info!(
        "Processing thumbnail image upload for post slug: {} by user: {}",
        slug, user_uuid
    );

    // 포스트 존재 확인 및 작성자 권한 검증 (user_id와 slug로 조회하므로 권한 검증 불필요)
    let post = repository_get_post_by_user_and_slug(conn, user_uuid, &slug).await?;

    // 필수 필드 검증
    let file_data = file_data.ok_or_else(|| {
        error!("No image file provided in multipart data");
        Errors::BadRequestError("Image file is required".to_string())
    })?;

    // 파일 크기 검증 (8MB)
    const MAX_FILE_SIZE: usize = 8 * 1024 * 1024;
    if file_data.len() > MAX_FILE_SIZE {
        return Err(Errors::BadRequestError(format!(
            "Image too large: {} bytes (max: {} bytes)",
            file_data.len(),
            MAX_FILE_SIZE
        )));
    }

    if file_data.is_empty() {
        return Err(Errors::BadRequestError(
            "Empty file not allowed".to_string(),
        ));
    }

    // Content-Type 설정 (기본값: image/jpeg)
    let content_type = content_type.unwrap_or_else(|| "image/jpeg".to_string());

    info!(
        "Processing thumbnail image upload: post_slug={}, user_uuid={}, content_type={}, size={} bytes",
        slug,
        user_uuid,
        content_type,
        file_data.len()
    );

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S_%3f");
    let filename = format!("thumbnail_{}", timestamp);

    // 태스크 큐에 업로드 요청
    queue_post_thumbnail_update(
        http_client,
        user_uuid,
        &post.id,
        &filename,
        file_data,
        &content_type,
    )
    .await
    .map_err(|e| {
        error!("Failed to queue thumbnail image upload task: {}", e);
        Errors::SysInternalError("Failed to queue thumbnail image upload task".to_string())
    })?;

    info!(
        "Thumbnail image upload task queued for post slug: {} by user: {}",
        slug, user_uuid
    );

    Ok(())
}
