use crate::config::db_config::DbConfig;
use reqwest::{Client, multipart};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tracing::{error, info, warn};
use uuid::Uuid;

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

/// 포스트 썸네일 이미지 업데이트를 태스크 서버에 요청 (기존 삭제 후 새 업로드)
pub async fn queue_post_thumbnail_update(
    http_client: &Client,
    user_uuid: &Uuid,
    post_id: &Uuid,
    filename: &str,
    file_data: Vec<u8>,
    content_type: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();

    info!(
        "Queuing async post thumbnail update task for user: {} post: {}, size: {} bytes",
        user_uuid,
        post_id,
        file_data.len()
    );

    // multipart form 생성
    let form = multipart::Form::new()
        .text("user_uuid", user_uuid.to_string())
        .text("post_id", post_id.to_string())
        .text("filename", filename.to_string())
        .part(
            "file",
            multipart::Part::bytes(file_data)
                .mime_str(content_type)?
                .file_name(filename.to_string()),
        );

    let response = http_client
        .put(&format!("{}/tasks/post/thumbnail/update", task_server_url))
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
        "Post thumbnail update task queued asynchronously with ID: {}",
        task_response.task_id
    );

    Ok(task_response.task_id)
}