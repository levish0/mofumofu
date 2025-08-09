use crate::dto::post::response::post_info::PostInfoResponse;
use crate::service::error::errors::Errors;
use sea_orm::{ConnectionTrait, TransactionTrait};
use crate::repository::post::get_post_by_handle_and_slug::repository_get_post_by_handle_and_slug;

pub async fn service_get_post_by_handle_and_slug<C>(
    conn: &C,
    handle: &str,
    slug: &str,
) -> Result<PostInfoResponse, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let post = repository_get_post_by_handle_and_slug(conn, handle, slug).await?;

    Ok(PostInfoResponse {
        title: post.title,
        summary: post.summary,
        content: post.content,
        user_id: post.user_id,
        created_at: post.created_at,
        updated_at: post.updated_at,
        published_at: None,
        like_count: post.like_count,
        comment_count: post.comment_count,
        view_count: post.view_count,
        slug: post.slug,
    })
}