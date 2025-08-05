use crate::config::db_config::DbConfig;
use reqwest::{Client, multipart};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use sea_orm::Iden;
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

/// 태스크 서버 URL을 캐시하는 정적 변수
static TASK_SERVER_URL: LazyLock<String> = LazyLock::new(|| {
    let config = DbConfig::get();
    format!(
        "http://{}:{}",
        config.task_server_host, config.task_server_port
    )
});

/// 태스크 서버 URL을 가져오는 함수
fn get_task_server_url() -> &'static str {
    &TASK_SERVER_URL
}

/// 비동기적으로 OAuth 프로필 이미지 업로드를 큐에 등록만 하고 즉시 반환
/// (실제 업로드는 백그라운드에서 처리되고, 재시도는 Celery가 담당)
pub async fn queue_oauth_avatar_upload(
    http_client: &Client,
    user_uuid: &Uuid,
    user_handle: &str,
    google_picture_url: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();

    info!(
        "Queuing async OAuth profile image upload task for user: {} ({})",
        user_uuid, user_handle
    );

    let request = ProfileImageUploadRequest {
        user_id: user_uuid.to_string(),
        image_url: google_picture_url.to_string(),
    };

    let response = http_client
        .post(&format!("{}/tasks/oauth/upload-avatar", task_server_url))
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
    filename: &str,
    file_data: Vec<u8>,
    content_type: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();

    info!(
        "Queuing async avatar upload task for user: {} ({}), size: {} bytes",
        user_uuid,
        user_handle,
        file_data.len()
    );

    // multipart form 생성
    let form = multipart::Form::new()
        .text("user_uuid", user_uuid.to_string())
        .text("user_handle", user_handle.to_string())
        .text("filename", filename.to_string())
        .part(
            "file",
            multipart::Part::bytes(file_data)
                .mime_str(content_type)?
                .file_name(filename.to_string()),
        );

    let response = http_client
        .post(&format!("{}/tasks/avatar/upload", task_server_url))
        .multipart(form)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
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
    filename: &str,
    file_data: Vec<u8>,
    content_type: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();

    info!(
        "Queuing async banner upload task for user: {} ({}), size: {} bytes",
        user_uuid,
        user_handle,
        file_data.len()
    );

    // multipart form 생성
    let form = multipart::Form::new()
        .text("user_uuid", user_uuid.to_string())
        .text("user_handle", user_handle.to_string())
        .text("filename", filename.to_string())
        .part(
            "file",
            multipart::Part::bytes(file_data)
                .mime_str(content_type)?
                .file_name(filename.to_string()),
        );

    let response = http_client
        .post(&format!("{}/tasks/banner/upload", task_server_url))
        .multipart(form)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Task queue request failed: {} - {}", status, error_text).into());
    }

    let task_response: TaskResponse = response.json().await?;
    info!(
        "Banner upload task queued asynchronously with ID: {}",
        task_response.task_id
    );

    Ok(task_response.task_id)
}

/// 사용자 아바타 이미지 삭제를 태스크 서버에 요청
pub async fn queue_user_avatar_delete(
    http_client: &Client,
    user_uuid: &Uuid,
    user_handle: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();

    info!(
        "Queuing async avatar delete task for user: {} ({})",
        user_uuid, user_handle
    );

    // multipart form 생성
    let form = multipart::Form::new()
        .text("user_uuid", user_uuid.to_string())
        .text("user_handle", user_handle.to_string());

    let response = http_client
        .delete(&format!("{}/tasks/avatar/delete", task_server_url))
        .multipart(form)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Task queue request failed: {} - {}", status, error_text).into());
    }

    let task_response: TaskResponse = response.json().await?;
    info!(
        "Avatar delete task queued asynchronously with ID: {}",
        task_response.task_id
    );

    Ok(task_response.task_id)
}

/// 사용자 배너 이미지 삭제를 태스크 서버에 요청
pub async fn queue_user_banner_delete(
    http_client: &Client,
    user_uuid: &Uuid,
    user_handle: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();

    info!(
        "Queuing async banner delete task for user: {} ({})",
        user_uuid, user_handle
    );

    // multipart form 생성
    let form = multipart::Form::new()
        .text("user_uuid", user_uuid.to_string())
        .text("user_handle", user_handle.to_string());

    let response = http_client
        .delete(&format!("{}/tasks/banner/delete", task_server_url))
        .multipart(form)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Task queue request failed: {} - {}", status, error_text).into());
    }

    let task_response: TaskResponse = response.json().await?;
    info!(
        "Banner delete task queued asynchronously with ID: {}",
        task_response.task_id
    );

    Ok(task_response.task_id)
}

/// 사용자 아바타 이미지 업데이트를 태스크 서버에 요청 (기존 삭제 후 새 업로드)
pub async fn queue_user_avatar_update(
    http_client: &Client,
    user_uuid: &Uuid,
    user_handle: &str,
    filename: &str,
    file_data: Vec<u8>,
    content_type: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();

    info!(
        "Queuing async avatar update task for user: {} ({}), size: {} bytes",
        user_uuid,
        user_handle,
        file_data.len()
    );

    // multipart form 생성
    let form = multipart::Form::new()
        .text("user_uuid", user_uuid.to_string())
        .text("user_handle", user_handle.to_string())
        .text("filename", filename.to_string())
        .part(
            "file",
            multipart::Part::bytes(file_data)
                .mime_str(content_type)?
                .file_name(filename.to_string()),
        );

    let response = http_client
        .put(&format!("{}/tasks/avatar/update", task_server_url))
        .multipart(form)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Task queue request failed: {} - {}", status, error_text).into());
    }

    let task_response: TaskResponse = response.json().await?;
    info!(
        "Avatar update task queued asynchronously with ID: {}",
        task_response.task_id
    );

    Ok(task_response.task_id)
}

/// 사용자 배너 이미지 업데이트를 태스크 서버에 요청 (기존 삭제 후 새 업로드)
pub async fn queue_user_banner_update(
    http_client: &Client,
    user_uuid: &Uuid,
    user_handle: &str,
    filename: &str,
    file_data: Vec<u8>,
    content_type: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();

    info!(
        "Queuing async banner update task for user: {} ({}), size: {} bytes",
        user_uuid,
        user_handle,
        file_data.len()
    );

    // multipart form 생성
    let form = multipart::Form::new()
        .text("user_uuid", user_uuid.to_string())
        .text("user_handle", user_handle.to_string())
        .text("filename", filename.to_string())
        .part(
            "file",
            multipart::Part::bytes(file_data)
                .mime_str(content_type)?
                .file_name(filename.to_string()),
        );

    let response = http_client
        .put(&format!("{}/tasks/banner/update", task_server_url))
        .multipart(form)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Task queue request failed: {} - {}", status, error_text).into());
    }

    let task_response: TaskResponse = response.json().await?;
    info!(
        "Banner update task queued asynchronously with ID: {}",
        task_response.task_id
    );

    Ok(task_response.task_id)
}
