use crate::repository::user::update_user::repository_update_user;
use crate::dto::user::internal::update_user::UpdateUserFields;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::utils::image_validator::{generate_image_hash, validate_and_get_image_info};
use crate::connection::cloudflare_r2::R2Client;
use reqwest::Client;
use sea_orm::{ConnectionTrait, TransactionTrait};
use tracing::{error, info, warn};
use uuid::Uuid;

const MAX_OAUTH_IMAGE_SIZE: usize = 4 * 1024 * 1024; // 8MB

pub async fn upload_oauth_avatar<C>(
    conn: &C,
    r2_client: &R2Client,
    http_client: &Client,
    user_uuid: &Uuid,
    user_handle: &str,
    image_url: &str,
) -> ServiceResult<String>
where
    C: ConnectionTrait + TransactionTrait,
{
    info!(
        "Processing OAuth avatar image upload for user: {} ({})",
        user_uuid, user_handle
    );

    // Validate image URL
    if !image_url.starts_with("http://") && !image_url.starts_with("https://") {
        return Err(Errors::BadRequestError("Invalid image URL".to_string()));
    }

    // Download image from OAuth provider
    let response = http_client
        .get(image_url)
        .send()
        .await
        .map_err(|e| {
            error!("Failed to download OAuth avatar image: {}", e);
            Errors::SysInternalError("Failed to download avatar image".to_string())
        })?;

    if !response.status().is_success() {
        return Err(Errors::SysInternalError("Failed to download avatar image".to_string()));
    }

    // Get image data and validate size
    let image_data = response.bytes().await.map_err(|e| {
        error!("Failed to read OAuth avatar image data: {}", e);
        Errors::SysInternalError("Failed to process avatar image".to_string())
    })?;

    // Validate image and get info
    let (content_type, extension) = validate_and_get_image_info(&image_data, MAX_OAUTH_IMAGE_SIZE)?;
    
    // Generate hash-based filename (same as profile/banner uploads)
    let hash = generate_image_hash(&image_data);
    let filename = format!("avatar_{}.{}", hash, extension);

    info!(
        "Uploading OAuth avatar: user_uuid={}, handle={}, filename={}, size={} bytes, content_type={}",
        user_uuid,
        user_handle,
        filename,
        image_data.len(),
        content_type
    );

    // Upload to R2 using hash-based path (consistent with profile uploads)
    let r2_key = format!("profiles/{}/avatar/{}", user_handle, filename);
    r2_client
        .upload_with_content_type(&r2_key, image_data.to_vec(), &content_type)
        .await
        .map_err(|e| {
            error!("Failed to upload OAuth avatar to R2: {}", e);
            Errors::SysInternalError("Failed to upload avatar image".to_string())
        })?;

    // Get public URL
    let public_url = r2_client.get_r2_public_url(&r2_key);

    // Update user profile image in database
    let update_fields = UpdateUserFields {
        profile_image: Some(Some(public_url.clone())),
        ..Default::default()
    };

    repository_update_user(conn, user_uuid, update_fields)
        .await
        .map_err(|e| {
            error!("Failed to update user profile image in database: {:?}", e);
            Errors::SysInternalError("Failed to update user profile".to_string())
        })?;

    info!("OAuth avatar image uploaded successfully: {}", public_url);
    Ok(filename)
}