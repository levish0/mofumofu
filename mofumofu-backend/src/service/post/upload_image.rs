use crate::service::error::errors::Errors;
use crate::utils::image_validator::{generate_image_hash, validate_and_get_image_info};
use axum::extract::Multipart;
use reqwest::Client;
use tracing::{error, info};

pub async fn service_upload_image(
    http_client: &Client,
    user_uuid: &str,
    mut multipart: Multipart,
) -> Result<String, Errors> {
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
            let hash_filename = format!("{}.{}", hash, extension);
            
            // Send to tasks service
            let form = reqwest::multipart::Form::new()
                .text("user_uuid", user_uuid.to_string())
                .text("filename", hash_filename.clone())
                .part("file", reqwest::multipart::Part::bytes(data.to_vec()).mime_str(&content_type)
                    .map_err(|e| Errors::BadRequestError(format!("Invalid MIME type: {}", e)))?);

            let response = http_client
                .post("http://localhost:7000/post/image/upload")
                .multipart(form)
                .send()
                .await
                .map_err(|e| {
                    error!("Failed to send request to tasks service: {}", e);
                    Errors::SysInternalError("Failed to upload image".to_string())
                })?;

            if !response.status().is_success() {
                error!("Tasks service returned error: {}", response.status());
                return Err(Errors::SysInternalError("Failed to upload image".to_string()));
            }

            // Return the hash-based filename immediately
            info!("Image upload queued with filename: {}", hash_filename);
            return Ok(hash_filename);
        }
    }

    Err(Errors::BadRequestError("No file found in request".to_string()))
}