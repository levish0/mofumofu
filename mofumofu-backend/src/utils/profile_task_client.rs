use crate::config::db_config::DbConfig;
use reqwest::{Client, multipart};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Serialize)]
struct ProfileImageUploadRequest {
    user_id: String,
    image_url: String,
}

#[derive(Deserialize)]
struct TaskResponse {
    task_id: String,
    status: String,
    message: String,
}

/// 비동기적으로 OAuth 프로필 이미지 업로드를 큐에 등록만 하고 즉시 반환
/// (실제 업로드는 백그라운드에서 처리되고, 재시도는 Celery가 담당)
pub async fn queue_oauth_profile_image_upload(
    http_client: &Client,
    user_uuid: &Uuid,
    user_handle: &str,
    google_picture_url: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let config = DbConfig::get();
    let task_server_url = format!(
        "http://{}:{}",
        config.task_server_host, config.task_server_port
    );

    info!(
        "Queuing async OAuth profile image upload task for user: {} ({})",
        user_uuid, user_handle
    );

    let request = ProfileImageUploadRequest {
        user_id: user_uuid.to_string(),
        image_url: google_picture_url.to_string(),
    };

    let response = http_client
        .post(&format!("{}/tasks/oauth/upload-profile-image", task_server_url))
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Task queue request failed: {}", response.status()).into());
    }

    let task_response: TaskResponse = response.json().await?;
    info!(
        "OAuth profile image upload task queued asynchronously with ID: {}",
        task_response.task_id
    );

    Ok(task_response.task_id)
}

/// 사용자가 업로드한 아바타 이미지를 태스크 서버에 전송
pub async fn queue_user_avatar_upload(
    http_client: &Client,
    user_uuid: &Uuid,
    user_handle: &str,
    file_data: Vec<u8>,
    content_type: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let config = DbConfig::get();
    let task_server_url = format!(
        "http://{}:{}",
        config.task_server_host, config.task_server_port
    );

    info!(
        "Queuing async avatar upload task for user: {} ({}), size: {} bytes",
        user_uuid, user_handle, file_data.len()
    );

    // multipart form 생성
    let form = multipart::Form::new()
        .text("user_uuid", user_uuid.to_string())
        .text("user_handle", user_handle.to_string())
        .part("file", multipart::Part::bytes(file_data)
            .mime_str(content_type)?
            .file_name("avatar"));

    let response = http_client
        .post(&format!("{}/tasks/avatar/upload", task_server_url))
        .multipart(form)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Task queue request failed: {} - {}", status, error_text).into());
    }

    let task_response: TaskResponse = response.json().await?;
    info!(
        "Avatar upload task queued asynchronously with ID: {}",
        task_response.task_id
    );

    Ok(task_response.task_id)
}

/// 사용자가 업로드한 배너 이미지를 태스크 서버에 전송  
pub async fn queue_user_banner_upload(
    http_client: &Client,
    user_uuid: &Uuid,
    user_handle: &str,
    file_data: Vec<u8>,
    content_type: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let config = DbConfig::get();
    let task_server_url = format!(
        "http://{}:{}",
        config.task_server_host, config.task_server_port
    );

    info!(
        "Queuing async banner upload task for user: {} ({}), size: {} bytes",
        user_uuid, user_handle, file_data.len()
    );

    // multipart form 생성
    let form = multipart::Form::new()
        .text("user_uuid", user_uuid.to_string())
        .text("user_handle", user_handle.to_string())
        .part("file", multipart::Part::bytes(file_data)
            .mime_str(content_type)?
            .file_name("banner"));

    let response = http_client
        .post(&format!("{}/tasks/banner/upload", task_server_url))
        .multipart(form)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Task queue request failed: {} - {}", status, error_text).into());
    }

    let task_response: TaskResponse = response.json().await?;
    info!(
        "Banner upload task queued asynchronously with ID: {}",
        task_response.task_id
    );

    Ok(task_response.task_id)
}
