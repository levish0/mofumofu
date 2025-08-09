use crate::dto::post::response::post_info::{PostInfoResponse, PostAuthor};
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
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
    
    // Get author information
    let user = repository_find_user_by_uuid(conn, &post.user_id).await?
        .ok_or(Errors::UserNotFound)?;

    Ok(PostInfoResponse {
        title: post.title,
        summary: post.summary,
        content: post.content,
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