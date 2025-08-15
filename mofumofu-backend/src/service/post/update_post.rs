use crate::dto::post::request::update_post::UpdatePostRequest;
use crate::entity::common::{ActionType, TargetType};
use crate::repository::hashtag::associate_post_hashtags::repository_associate_post_hashtags;
use crate::repository::hashtag::remove_post_hashtags::repository_remove_post_hashtags;
use crate::repository::post::update_post::repository_update_post;
use crate::repository::system_events::log_event::repository_log_event;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::microservices::search_client;
use crate::microservices::markdown_client::queue_render_markdown;
use sea_orm::{ConnectionTrait, TransactionTrait};
use tracing::{info, warn};
use uuid::Uuid;

pub async fn service_update_post<C>(
    conn: &C,
    http_client: &reqwest::Client,
    payload: UpdatePostRequest,
    user_uuid: &Uuid,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    let updated_post = repository_update_post(&txn, payload.clone(), user_uuid).await?;

    if let Some(ref hashtags) = payload.hashtags {
        repository_remove_post_hashtags(&txn, updated_post.id).await?;
        
        if !hashtags.is_empty() {
            repository_associate_post_hashtags(&txn, updated_post.id, hashtags, *user_uuid).await?;
        }
    }

    txn.commit().await?;

    if let Err(e) = search_client::queue_update_post(http_client, &updated_post.id).await {
        warn!("Failed to queue post search update task: {}", e);
    }

    if payload.content.is_some() {
        info!("글 수정 완료, 마크다운 렌더링 태스크 큐에 추가 (post_id: {})", updated_post.id);
        if let Err(e) = queue_render_markdown(http_client, &updated_post.id, &updated_post.content).await {
            warn!("Failed to queue markdown rendering task for post {}: {}", updated_post.id, e);
        } else {
            info!("마크다운 렌더링 태스크 큐에 추가 완료 (post_id: {})", updated_post.id);
        }
    }

    repository_log_event(
        conn,
        Some(*user_uuid),
        ActionType::PostUpdated,
        Some(updated_post.id),
        Some(TargetType::Post),
        None,
    )
    .await;

    Ok(())
}