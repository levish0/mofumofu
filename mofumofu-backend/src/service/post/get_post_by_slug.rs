use crate::dto::user::response::info::UserInfoResponse;
use crate::repository::user::get_user_by_handle::repository_get_user_by_handle;
use crate::service::error::errors::Errors;
use sea_orm::{ConnectionTrait, TransactionTrait};
use crate::dto::post::response::post_info::PostInfoResponse;
use crate::repository::post::get_post_by_slug::repository_get_post_by_slug;

pub async fn service_get_post_by_slug<C>(
    conn: &C,
    slug: &str,
) -> Result<PostInfoResponse, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let post = repository_get_post_by_slug(conn, slug).await?;

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
