use chrono::{DateTime, Utc};
use crate::connection::meilisearch::MeilisearchClient;
use crate::dto::post::request::{PostSortOrder, SearchPostsRequest};
use crate::dto::post::response::{GetPostsResponse, PostListItem};
use crate::service::error::errors::Errors;
use crate::service::meilisearch::post_indexer;
use sea_orm::{ConnectionTrait, TransactionTrait};

pub async fn service_search_posts<C>(
    _conn: &C,
    meilisearch: &MeilisearchClient,
    request: SearchPostsRequest,
) -> Result<GetPostsResponse, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let target_page = request.target_page.unwrap_or(1);
    let page_size = request.page_size.unwrap_or(20);
    let pages_around = request.pages_around.unwrap_or(2);
    let sort_order = request.sort.unwrap_or(PostSortOrder::Latest);

    // 날짜를 Unix timestamp로 변환
    let date_from = request.date_from.map(|dt| dt.timestamp());
    let date_to = request.date_to.map(|dt| dt.timestamp());

    // 정렬 문자열 변환
    let sort_str = match sort_order {
        PostSortOrder::Popular => "popular",
        PostSortOrder::Oldest => "oldest",
        PostSortOrder::Latest => "latest",
    };

    // 페이지 범위 계산 (around 방식)
    let start_page = if target_page > pages_around {
        target_page - pages_around
    } else {
        1
    };
    let end_page = target_page + pages_around;
    let total_pages_to_fetch = end_page - start_page + 1;
    let total_items_to_fetch = total_pages_to_fetch * page_size;

    // Meilisearch에서 검색 (around 방식으로)
    let meilisearch_posts = post_indexer::search_posts(
        meilisearch,
        request.query.as_deref(),
        request.hashtags.as_deref(),
        request.user_handle.as_deref(),
        date_from,
        date_to,
        request.min_likes,
        sort_str,
        start_page, // 시작 페이지부터 가져오기
        total_items_to_fetch,
    )
    .await
    .map_err(|e| {
        tracing::warn!("Meilisearch search failed: {}", e);
        Errors::SysInternalError(format!("Search service error: {}", e))
    })?;

    if meilisearch_posts.is_empty() {
        return Ok(GetPostsResponse {
            posts: Vec::new(),
            current_page: target_page,
            page_size,
            has_more: false,
            total_count: Some(0),
        });
    }

    // MeilisearchPost를 PostListItem으로 변환
    let post_items: Vec<PostListItem> = meilisearch_posts
        .into_iter()
        .map(|mpost| {
            // created_at을 Unix timestamp에서 DateTime으로 변환
            let created_at = DateTime::from_timestamp(mpost.created_at, 0)
                .unwrap_or_else(Utc::now);

            PostListItem {
                id: mpost.id.parse().unwrap_or_default(),
                title: mpost.title,
                summary: mpost.summary,
                thumbnail_image: mpost.thumbnail_image,
                user_handle: mpost.user_handle,
                user_name: mpost.user_name,
                user_avatar: mpost.user_avatar,
                created_at,
                like_count: mpost.like_count,
                comment_count: mpost.comment_count,
                view_count: mpost.view_count,
                slug: mpost.slug,
                hashtags: mpost.hashtags,
            }
        })
        .collect();

    // 다음 페이지가 있는지 확인 (around 방식)
    let has_more = post_items.len() == total_items_to_fetch as usize;

    Ok(GetPostsResponse {
        posts: post_items,
        current_page: target_page,
        page_size,
        has_more,
        total_count: None, // Meilisearch에서 총 개수를 가져오려면 별도 요청 필요
    })
}
