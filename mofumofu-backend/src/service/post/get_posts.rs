use crate::dto::post::request::{GetPostsRequest, PostSortOrder};
use crate::dto::post::response::{GetPostsResponse, PostListItem};
use crate::repository::post::get_posts::{repository_get_posts, repository_get_posts_count};
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::Errors;
use sea_orm::{ConnectionTrait, TransactionTrait};

pub async fn service_get_posts<C>(
    conn: &C,
    request: GetPostsRequest,
) -> Result<GetPostsResponse, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let page = request.page.unwrap_or(1);
    let page_size = request.page_size.unwrap_or(8);
    let sort_order = request.sort.unwrap_or(PostSortOrder::Latest);

    // 포스트 가져오기
    let posts = repository_get_posts(conn, page, page_size, &sort_order).await?;
    
    // 각 포스트에 대해 유저 정보를 가져와서 PostListItem으로 변환
    let mut post_items = Vec::new();
    
    for post in posts {
        let user = repository_find_user_by_uuid(conn, &post.user_id).await?
            .ok_or(Errors::UserNotFound)?;
        
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
        });
    }

    // 다음 페이지가 있는지 확인
    let has_more = post_items.len() == page_size as usize;

    // 전체 개수 (선택적)
    let total_count = Some(repository_get_posts_count(conn).await?);

    Ok(GetPostsResponse {
        posts: post_items,
        current_page: page,
        page_size,
        has_more,
        total_count,
    })
}