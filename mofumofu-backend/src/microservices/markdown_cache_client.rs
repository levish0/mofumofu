use crate::config::db_config::DbConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tracing::{error, info, warn};

#[derive(Serialize)]
struct MarkdownCacheRequest {
    post_id: String,
}

#[derive(Serialize)]
struct MarkdownRenderRequest {
    post_id: String,
    content: String,
    cache_ttl: Option<i32>,
}

#[derive(Deserialize)]
struct TocItem {
    level: i32,
    text: String,
    id: String,
}

#[derive(Deserialize)]
struct MarkdownData {
    #[serde(rename = "html_content")]
    html_content: String,
    #[serde(rename = "toc_items")]
    toc_items: Vec<TocItem>,
}

#[derive(Deserialize)]
struct MarkdownCacheResponse {
    success: bool,
    data: Option<MarkdownData>,
    cached: Option<bool>,
    cache_key: Option<String>,
    message: Option<String>,
}

#[derive(Debug)]
pub struct RenderedMarkdown {
    pub html_content: String,
    pub toc_items: Vec<TableOfContentsItem>,
}

#[derive(Debug)]
pub struct TableOfContentsItem {
    pub level: i32,
    pub text: String,
    pub id: String,
}

/// 태스크 서비스 URL을 캐시하는 정적 변수
static TASKS_SERVICE_URL: LazyLock<String> = LazyLock::new(|| {
    let config = DbConfig::get();
    format!(
        "http://{}:{}",
        config.task_server_host, config.task_server_port
    )
});

/// 태스크 서비스 URL을 가져오는 함수
fn get_tasks_service_url() -> &'static str {
    &TASKS_SERVICE_URL
}

/// 캐시된 마크다운 렌더링 결과를 Redis에서 직접 조회 (빠른 직접 조회)
pub async fn get_cached_markdown(
    redis_conn: &mut redis::aio::ConnectionManager,
    post_id: &str,
) -> Result<Option<RenderedMarkdown>, Box<dyn std::error::Error + Send + Sync>> {
    use redis::AsyncCommands;
    use crate::config::db_config::DbConfig;
    
    // Redis DB 1에 직접 연결 (AppState의 Redis는 DB 0이므로 새로 연결)
    let redis_url = format!(
        "redis://{}:{}/1",
        &DbConfig::get().redis_host,
        &DbConfig::get().redis_port,
    );
    let client = redis::Client::open(redis_url)?;
    let mut conn = redis::aio::ConnectionManager::new(client).await?;
    
    let cache_key = format!("markdown:rendered:post:{}", post_id);
    info!("Redis DB 1에서 캐시된 마크다운 조회: key={}", cache_key);

    let cached_result: Option<String> = conn.get(&cache_key).await?;

    match cached_result {
        Some(json_data) => {
            match serde_json::from_str::<MarkdownData>(&json_data) {
                Ok(data) => {
                    let toc_items: Vec<TableOfContentsItem> = data
                        .toc_items
                        .into_iter()
                        .map(|item| TableOfContentsItem {
                            level: item.level,
                            text: item.text,
                            id: item.id,
                        })
                        .collect();

                    info!(
                        "캐시된 마크다운 조회 성공, TOC 아이템: {} 개", 
                        toc_items.len()
                    );

                    Ok(Some(RenderedMarkdown {
                        html_content: data.html_content,
                        toc_items,
                    }))
                }
                Err(e) => {
                    warn!("캐시된 데이터 파싱 실패: {}", e);
                    Ok(None)
                }
            }
        }
        None => {
            info!("캐시된 마크다운 없음: post_id={}", post_id);
            Ok(None)
        }
    }
}

/// 마크다운을 렌더링하고 캐시 (캐시가 있으면 즉시 반환)
pub async fn render_and_cache_markdown(
    http_client: &Client,
    post_id: &str,
    content: &str,
    cache_ttl: Option<i32>,
) -> Result<RenderedMarkdown, Box<dyn std::error::Error + Send + Sync>> {
    let service_url = get_tasks_service_url();
    
    info!("마크다운 렌더링 및 캐시 요청: post_id={}, 길이: {} chars", post_id, content.len());

    let request = MarkdownRenderRequest {
        post_id: post_id.to_string(),
        content: content.to_string(),
        cache_ttl,
    };

    let response = http_client
        .post(&format!("{}/tasks/markdown/render", service_url))
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        error!("마크다운 렌더링 요청 실패: {} - {}", status, error_text);
        return Err(format!("마크다운 렌더링 요청 실패: {} - {}", status, error_text).into());
    }

    let render_response: MarkdownCacheResponse = response.json().await?;

    if !render_response.success {
        let error_msg = render_response.message.unwrap_or_else(|| "Unknown error".to_string());
        error!("마크다운 렌더링 실패: {}", error_msg);
        return Err(format!("마크다운 렌더링 실패: {}", error_msg).into());
    }

    let data = render_response.data.ok_or("렌더링 응답에서 데이터 누락")?;

    let toc_items: Vec<TableOfContentsItem> = data
        .toc_items
        .into_iter()
        .map(|item| TableOfContentsItem {
            level: item.level,
            text: item.text,
            id: item.id,
        })
        .collect();

    let was_cached = render_response.cached.unwrap_or(false);
    info!(
        "마크다운 렌더링 완료 (캐시: {}), TOC 아이템: {} 개", 
        if was_cached { "히트" } else { "미스" },
        toc_items.len()
    );

    Ok(RenderedMarkdown {
        html_content: data.html_content,
        toc_items,
    })
}

/// 마크다운 캐시 무효화
pub async fn invalidate_markdown_cache(
    http_client: &Client,
    post_id: &str,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let service_url = get_tasks_service_url();
    
    info!("마크다운 캐시 무효화 요청: post_id={}", post_id);

    let request = MarkdownCacheRequest {
        post_id: post_id.to_string(),
    };

    let response = http_client
        .post(&format!("{}/tasks/markdown/invalidate-cache", service_url))
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        warn!("캐시 무효화 요청 실패: {} - {}", status, error_text);
        return Ok(false);
    }

    let invalidate_response: MarkdownCacheResponse = response.json().await?;

    if invalidate_response.success {
        info!("마크다운 캐시 무효화 완료");
        Ok(true)
    } else {
        warn!("마크다운 캐시 무효화 실패: {}", invalidate_response.message.unwrap_or_default());
        Ok(false)
    }
}

/// 마크다운 캐시 서비스 헬스 체크
pub async fn check_markdown_cache_service_health(
    http_client: &Client,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let service_url = get_tasks_service_url();
    
    let response = http_client
        .get(&format!("{}/tasks/markdown/health", service_url))
        .send()
        .await?;

    Ok(response.status().is_success())
}