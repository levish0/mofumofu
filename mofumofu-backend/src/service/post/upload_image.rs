use crate::connection::cloudflare_r2::R2Client;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::utils::image_validator::{generate_image_hash, validate_and_get_image_info};
use axum::extract::Multipart;
use tracing::{error, info};
use uuid::Uuid;

pub async fn service_upload_image(
    r2_client: &R2Client,
    user_uuid: &Uuid,
    mut multipart: Multipart,
) -> ServiceResult<String> {
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

            // Validate image and get info (8MB limit for post images)
            const MAX_POST_IMAGE_SIZE: usize = 8 * 1024 * 1024;
            let (content_type, extension) = validate_and_get_image_info(&data, MAX_POST_IMAGE_SIZE)?;
            
            // Generate hash-based filename
            let hash = generate_image_hash(&data);
            let filename = format!("post_image_{}.{}", hash, extension);

            info!(
                "Processing post image upload: user_uuid={}, filename={}, content_type={}, size={} bytes",
                user_uuid,
                filename,
                content_type,
                data.len()
            );

            // Upload to R2
            let key = format!("post-images/{}", filename);
            match r2_client.upload_with_content_type(&key, data.to_vec(), &content_type).await {
                Ok(_) => {
                    info!("Successfully uploaded post image to R2: {}", filename);
                    return Ok(filename);
                }
                Err(e) => {
                    error!("Failed to upload post image to R2: {}", e);
                    return Err(Errors::SysInternalError("Failed to upload image to storage".to_string()));
                }
            }
        }
    }

    Err(Errors::BadRequestError("No file found in request".to_string()))
}