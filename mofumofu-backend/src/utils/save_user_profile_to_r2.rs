use std::time::Duration;
use reqwest::Client;
use tracing::info;
use crate::connection::cloudflare_r2::R2Client;

const MAX_IMAGE_SIZE: usize = 8 * 1024 * 1024;

pub async fn save_user_profile_to_r2(
    http_client: &Client,
    r2_client: &R2Client,
    user_id: &str,
    google_picture_url: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    info!("Downloading profile image from Google for user: {}", user_id);

    // Google에서 이미지 다운로드
    let response = http_client
        .get(google_picture_url)
        .timeout(Duration::from_secs(15))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Failed to download image: {}", response.status()).into());
    }

    // Content-Type 추출 및 검증
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|ct| ct.to_str().ok())
        .unwrap_or("image/jpeg")
        .to_string();

    // 지원되는 이미지 타입인지 확인
    if !is_supported_image_type(&content_type) {
        return Err(format!("Unsupported image type: {}", content_type).into());
    }

    // Content-Length 확인 (헤더에 있는 경우)
    if let Some(content_length) = response.headers().get("content-length") {
        if let Ok(length_str) = content_length.to_str() {
            if let Ok(length) = length_str.parse::<usize>() {
                if length > MAX_IMAGE_SIZE {
                    return Err(format!("Image too large: {} bytes (max: {} bytes)", length, MAX_IMAGE_SIZE).into());
                }
            }
        }
    }

    let image_data = response.bytes().await?;

    // 실제 이미지 크기 체크
    if image_data.len() > MAX_IMAGE_SIZE {
        return Err(format!("Profile image too large: {} bytes (max: {} bytes)", image_data.len(), MAX_IMAGE_SIZE).into());
    }

    // 최소 크기 체크 (빈 파일 방지)
    if image_data.is_empty() {
        return Err("Empty image file".into());
    }

    // R2 저장 경로 (사용자 ID 기반)
    let r2_key = format!("profiles/{}/avatar", user_id);

    // R2에 업로드
    r2_client
        .upload_with_content_type(&r2_key, image_data.to_vec(), &content_type)
        .await
        .map_err(|e| format!("R2 upload failed: {}", e))?;

    // 공개 URL 생성
    let public_url = r2_client.get_r2_public_url(&r2_key);

    info!("Profile image uploaded to R2: {}", public_url);
    Ok(public_url)
}

fn is_supported_image_type(content_type: &str) -> bool {
    matches!(content_type,
        "image/jpeg" | "image/jpg" | "image/png" | "image/webp" | "image/gif"
    )
}