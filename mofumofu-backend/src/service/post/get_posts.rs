use crate::connection::meilisearch::MeilisearchClient;
use crate::dto::post::request::{GetPostsRequest, PostSortOrder};
use crate::dto::post::response::{GetPostsResponse, PostListItem};
use crate::repository::hashtag::get_hashtags_by_post::repository_get_hashtags_by_posts;
use crate::repository::post::get_posts::{repository_get_posts, repository_get_posts_count};
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::Errors;
use crate::service::meilisearch::post_indexer;
use sea_orm::{ConnectionTrait, TransactionTrait};

pub async fn service_get_posts<C>(
    conn: &C,
    meilisearch: &MeilisearchClient,
    request: GetPostsRequest,
) -> Result<GetPostsResponse, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let page = request.page.unwrap_or(1);
    let page_size = request.page_size.unwrap_or(8);
    let sort_order = request.sort.unwrap_or(PostSortOrder::Latest);

    // 정렬 문자열 변환
    let sort_str = match sort_order {
        PostSortOrder::Popular => "popular",
        PostSortOrder::Oldest => "oldest",
        PostSortOrder::Latest => "latest",
    };

    // Meilisearch로 포스트 검색 (빠른 성능)
    let meilisearch_posts = match post_indexer::search_posts(
        meilisearch,
        None, // query (empty for general listing)
        None, // hashtags
        None, // user_handle
        None, // date_from
        None, // date_to
        None, // min_likes
        sort_str,
        page,
        page_size,
    )
    .await
    {
        Ok(posts) => posts,
        Err(e) => {
            // Meilisearch 오류 시 DB로 폴백
            tracing::warn!("Meilisearch fallback for get_posts: {}", e);
            let posts = repository_get_posts(conn, page, page_size, &sort_order).await?;

            if posts.is_empty() {
                return Ok(GetPostsResponse {
                    posts: Vec::new(),
                    current_page: page,
                    page_size,
                    has_more: false,
                    total_count: Some(0),
                });
            }

            // DB 결과를 PostListItem으로 변환 (기존 로직)
            let post_ids: Vec<uuid::Uuid> = posts.iter().map(|p| p.id).collect();
            let post_hashtags_map = repository_get_hashtags_by_posts(conn, &post_ids)
                .await?
                .into_iter()
                .collect::<std::collections::HashMap<_, _>>();

            let mut post_items = Vec::new();
            for post in posts {
                let user = repository_find_user_by_uuid(conn, &post.user_id)
                    .await?
                    .ok_or(Errors::UserNotFound)?;

                let hashtags = post_hashtags_map
                    .get(&post.id)
                    .map(|tags| tags.iter().map(|tag| tag.name.clone()).collect())
                    .unwrap_or_else(Vec::new);

                post_items.push(PostListItem {
                    id: post.id,
                    title: post.title,
                    summary: post.summary,
                    thumbnail_image: post.thumbnail_image,
                    user_handle: user.handle,
                    user_name: user.name,
                    user_avatar: user.profile_image,
                    created_at: post.created_at,
                    like_count: post.like_count,
                    comment_count: post.comment_count,
                    view_count: post.view_count,
                    slug: post.slug,
                    hashtags,
                });
            }

            let has_more = post_items.len() == page_size as usize;
            let total_count = Some(repository_get_posts_count(conn).await?);

            return Ok(GetPostsResponse {
                posts: post_items,
                current_page: page,
                page_size,
                has_more,
                total_count,
            });
        }
    };

    if meilisearch_posts.is_empty() {
        return Ok(GetPostsResponse {
            posts: Vec::new(),
            current_page: page,
            page_size,
            has_more: false,
            total_count: Some(0),
        });
    }

    // MeilisearchPost를 PostListItem으로 변환
    let post_items: Vec<PostListItem> = meilisearch_posts
        .into_iter()
        .map(|mpost| {
            let created_at = chrono::DateTime::from_timestamp(mpost.created_at, 0)
                .unwrap_or_else(chrono::Utc::now);

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

    let has_more = post_items.len() == page_size as usize;

    Ok(GetPostsResponse {
        posts: post_items,
        current_page: page,
        page_size,
        has_more,
        total_count: None, // Meilisearch에서 총 개수를 가져오려면 별도 요청 필요
    })
}
