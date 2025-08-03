use crate::config::db_config::DbConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

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

/// 비동기적으로 프로필 이미지 업로드를 큐에 등록만 하고 즉시 반환
/// (실제 업로드는 백그라운드에서 처리되고, 재시도는 Celery가 담당)
pub async fn queue_profile_image_upload(
    http_client: &Client,
    user_id: &str,
    google_picture_url: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let config = DbConfig::get();
    let task_server_url = format!(
        "http://{}:{}",
        config.task_server_host, config.task_server_port
    );

    info!(
        "Queuing async profile image upload task for user: {}",
        user_id
    );

    let request = ProfileImageUploadRequest {
        user_id: user_id.to_string(),
        image_url: google_picture_url.to_string(),
    };

    let response = http_client
        .post(&format!("{}/tasks/profile/upload-image", task_server_url))
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Task queue request failed: {}", response.status()).into());
    }

    let task_response: TaskResponse = response.json().await?;
    info!(
        "Profile image upload task queued asynchronously with ID: {}",
        task_response.task_id
    );

    Ok(task_response.task_id)
}
