use crate::dto::post::response::post_info::{PostAuthor, PostInfoResponse, TocItem};
use crate::microservices::markdown_client::render_markdown;
use crate::repository::post::get_post_by_handle_and_slug::repository_get_post_by_handle_and_slug;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::Errors;
use reqwest::Client;
use sea_orm::{ConnectionTrait, Iden, TransactionTrait};

pub async fn service_get_post_by_handle_and_slug<C>(
    conn: &C,
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

    // Render markdown to HTML
    let rendered_result = render_markdown(http_client, &post.content).await
        .map_err(|_| Errors::SysInternalError("".to_string()))?;

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
