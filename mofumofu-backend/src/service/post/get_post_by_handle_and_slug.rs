use crate::dto::post::response::post_info::{PostAuthor, PostInfoResponse, TocItem};
use crate::microservices::markdown_cache_client::{get_cached_markdown, render_and_cache_markdown};
use crate::repository::post::get_post_by_handle_and_slug::repository_get_post_by_handle_and_slug;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::Errors;
use redis::aio::ConnectionManager;
use reqwest::Client;
use sea_orm::{ConnectionTrait, Iden, TransactionTrait};
use tracing::{info, warn};

pub async fn service_get_post_by_handle_and_slug<C>(
    conn: &C,
    redis_conn: &mut ConnectionManager,
    http_client: &Client,
    handle: &str,
    slug: &str,
) -> Result<PostInfoResponse, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let post = repository_get_post_by_handle_and_slug(conn, handle, slug).await?;

    // Get author information
    let user = repository_find_user_by_uuid(conn, &post.user_id)
        .await?
        .ok_or(Errors::UserNotFound)?;

    // 마크다운 캐시 렌더링 시도
    info!("포스트 마크다운 렌더링 시작 (handle: {}, slug: {})", handle, slug);
    
    let rendered_result = match get_cached_markdown(redis_conn, &post.id.to_string()).await {
        Ok(Some(cached_result)) => {
            info!("캐시된 마크다운 사용 (post_id: {}, handle: {}, slug: {})", post.id, handle, slug);
            cached_result
        }
        Ok(None) => {
            info!("캐시 미스 - 새로 렌더링 (post_id: {}, handle: {}, slug: {})", post.id, handle, slug);
            render_and_cache_markdown(http_client, &post.id.to_string(), &post.content, Some(86400)).await
                .map_err(|e| {
                    warn!("마크다운 렌더링 실패: {}", e);
                    Errors::SysInternalError("마크다운 렌더링 실패".to_string())
                })?
        }
        Err(e) => {
            warn!("캐시 조회 중 오류, 직접 렌더링 시도: {}", e);
            render_and_cache_markdown(http_client, &post.id.to_string(), &post.content, Some(86400)).await
                .map_err(|e| {
                    warn!("마크다운 렌더링 실패: {}", e);
                    Errors::SysInternalError("마크다운 렌더링 실패".to_string())
                })?
        }
    };

    let toc_items: Vec<TocItem> = rendered_result
        .toc_items
        .into_iter()
        .map(|item| TocItem {
            level: item.level,
            text: item.text,
            id: item.id,
        })
        .collect();

    Ok(PostInfoResponse {
        title: post.title,
        summary: post.summary,
        content: post.content,
        rendered: rendered_result.html_content,
        toc_items,
        author: PostAuthor {
            handle: user.handle,
            name: user.name,
            profile_image: user.profile_image,
        },
        created_at: post.created_at,
        updated_at: post.updated_at,
        published_at: None,
        like_count: post.like_count,
        comment_count: post.comment_count,
        view_count: post.view_count,
        slug: post.slug,
        tags: Vec::new(), // TODO: Implement tags system
    })
}
