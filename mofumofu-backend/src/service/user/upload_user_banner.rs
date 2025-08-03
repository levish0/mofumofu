use crate::service::error::errors::Errors;
use crate::service::user::get_user_by_uuid;
use crate::utils::profile_task_client::{ queue_user_profile_upload};
use axum::extract::Multipart;
use reqwest::Client;
use sea_orm::ConnectionTrait;
use tracing::{error, info, warn};
use uuid::Uuid;
use crate::repository::user::get_user_by_uuid::repository_get_user_by_uuid;

pub async fn service_upload_user_profile<C>(
    conn: &C,
    http_client: &Client,
    user_uuid: &Uuid,
    mut multipart: Multipart,
    image_type: &str, // "profile" or "banner"
) -> Result<(), Errors> 
where
    C: ConnectionTrait,
{
    info!("Processing {} image upload for user: {}", image_type, user_uuid);

    // UUID로 사용자 정보 조회
    let user = repository_get_user_by_uuid(conn, user_uuid).await?;

    let mut file_data: Option<Vec<u8>> = None;
    let mut content_type: Option<String> = None;

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
            _ => {
                warn!("Unknown field in multipart: {}", field_name);
            }
        }
    }

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
        return Err(Errors::BadRequestError("Empty file not allowed".to_string()));
    }

    // Content-Type 설정 (기본값: image/jpeg)
    let content_type = content_type.unwrap_or_else(|| "image/jpeg".to_string());

    info!(
        "Processing {} image upload: user_uuid={}, content_type={}, size={} bytes",
        image_type,
        user_uuid,
        content_type,
        file_data.len()
    );

    // 태스크 큐에 업로드 요청
    queue_user_profile_upload(
        http_client,
        &user_uuid,
        &user.handle,
        file_data,
        &content_type,
        image_type,
    )
    .await
    .map_err(|e| {
        error!("Failed to queue {} image upload task: {}", image_type, e);
        Errors::SysInternalError(format!("Failed to queue {} image upload task", image_type))
    })?;

    info!("{} image upload task queued for user: {}", image_type, user_uuid);

    Ok(())
}