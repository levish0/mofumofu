use crate::repository::post::get_post_by_user_and_slug::repository_get_post_by_user_and_slug;
use crate::repository::post::update_post_thumbnail::repository_update_post_thumbnail;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::utils::image_validator::{generate_image_hash, validate_and_get_image_info};
use crate::connection::cloudflare_r2::R2Client;
use axum::extract::Multipart;
use sea_orm::{ConnectionTrait, TransactionTrait};
use tracing::{error, info, warn};
use uuid::Uuid;

pub async fn service_update_post_thumbnail<C>(
    conn: &C,
    r2_client: &R2Client,
    user_uuid: &Uuid,
    mut multipart: Multipart,
) -> ServiceResult<String>
where
    C: ConnectionTrait + TransactionTrait,
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

    // Validate image and get info (8MB limit for thumbnails)
    const MAX_THUMBNAIL_SIZE: usize = 8 * 1024 * 1024;
    let (content_type, extension) = validate_and_get_image_info(&file_data, MAX_THUMBNAIL_SIZE)?;
    
    // Generate hash-based filename
    let hash = generate_image_hash(&file_data);
    let filename = format!("thumbnail_{}.{}", hash, extension);

    info!(
        "Processing thumbnail image upload: post_slug={}, user_uuid={}, filename={}, content_type={}, size={} bytes",
        slug,
        user_uuid,
        filename,
        content_type,
        file_data.len()
    );

    // Delete existing thumbnail if exists
    if let Some(existing_thumbnail_url) = &post.thumbnail_image {
        if !existing_thumbnail_url.is_empty() {
            // Extract key from URL and delete from R2
            let key = format!("posts/{}/thumbnail/{}", post.id, filename);
            if let Err(e) = r2_client.delete(&key).await {
                warn!("Failed to delete existing thumbnail from R2: {}", e);
            }
        }
    }

    // Upload to R2
    let r2_key = format!("posts/{}/thumbnail/{}", post.id, filename);
    r2_client.upload_with_content_type(&r2_key, file_data, &content_type)
        .await
        .map_err(|e| {
            error!("Failed to upload thumbnail to R2: {}", e);
            Errors::SysInternalError("Failed to upload thumbnail image".to_string())
        })?;

    // Get public URL
    let public_url = r2_client.get_r2_public_url(&r2_key);

    // Update post thumbnail in database
    repository_update_post_thumbnail(conn, &post.id, Some(public_url.clone()))
        .await
        .map_err(|e| {
            error!("Failed to update post thumbnail in database: {:?}", e);
            Errors::SysInternalError("Failed to update post thumbnail".to_string())
        })?;

    info!("Thumbnail image uploaded successfully: {}", public_url);
    Ok(public_url)
}
