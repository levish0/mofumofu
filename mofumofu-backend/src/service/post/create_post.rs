use crate::dto::post::request::create_post::CreatePostRequest;
use crate::entity::common::{ActionType, TargetType};
use crate::repository::hashtag::associate_post_hashtags::repository_associate_post_hashtags;
use crate::repository::post::create_post::repository_create_post;
use crate::repository::system_events::log_event::repository_log_event;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::microservices::search_client;
use crate::microservices::markdown_client::queue_render_markdown;
use sea_orm::{ConnectionTrait, TransactionTrait};
use tracing::{info, warn};
use uuid::Uuid;

pub async fn service_create_post<C>(
    conn: &C,
    http_client: &reqwest::Client,
    payload: CreatePostRequest,
    user_uuid: &Uuid,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    let hashtags = payload.hashtags.clone();

    let post = CreatePostRequest {
        title: payload.title,
        summary: payload.summary,
        content: payload.content,
        slug: payload.slug,
        hashtags: payload.hashtags,
    };

    let created_post = repository_create_post(&txn, post, user_uuid).await?;

    let hashtag_ids = if let Some(ref tags) = hashtags {
        if !tags.is_empty() {
            repository_associate_post_hashtags(&txn, created_post.id, tags, *user_uuid).await?
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    // Commit the transaction
    txn.commit().await?;

    // Python 태스크로 색인 요청 (DB 트랜잭션 외부에서 실행)
    if let Err(e) = search_client::queue_index_post(http_client, &created_post.id).await {
        warn!("Failed to queue post indexing task: {}", e);
    }

    // 마크다운 렌더링 태스크 큐에 추가 (백그라운드, 실패해도 무시)
    info!("글 생성 완료, 마크다운 렌더링 태스크 큐에 추가 (post_id: {})", created_post.id);
    if let Err(e) = queue_render_markdown(http_client, &created_post.id, &created_post.content).await {
        warn!("Failed to queue markdown rendering task for post {}: {}", created_post.id, e);
        // 실패해도 글 생성은 성공으로 처리
    } else {
        info!("마크다운 렌더링 태스크 큐에 추가 완료 (post_id: {})", created_post.id);
    }

    // 이벤트 로깅 - 포스트 생성
    repository_log_event(
        conn,
        Some(*user_uuid),
        ActionType::PostCreated,
        Some(created_post.id),
        Some(TargetType::Post),
        None,
    )
    .await;

    // 해시태그 사용 이벤트 로깅
    for hashtag_id in hashtag_ids {
        repository_log_event(
            conn,
            Some(*user_uuid),
            ActionType::HashtagUsed,
            Some(hashtag_id),
            Some(TargetType::Hashtag),
            None,
        )
        .await;
    }

    Ok(())
}
