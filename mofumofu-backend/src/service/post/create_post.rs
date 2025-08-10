use crate::connection::meilisearch::{MeilisearchClient, MeilisearchPost};
use crate::dto::post::request::create_post::CreatePostRequest;
use crate::repository::hashtag::associate_post_hashtags::repository_associate_post_hashtags;
use crate::repository::hashtag::get_hashtags_by_post::repository_get_hashtags_by_post;
use crate::repository::post::create_post::repository_create_post;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::Errors;
use crate::service::meilisearch::post_indexer;
use sea_orm::{ConnectionTrait, TransactionTrait};
use tracing::warn;
use uuid::Uuid;

pub async fn service_create_post<C>(
    conn: &C,
    meilisearch: &MeilisearchClient,
    payload: CreatePostRequest,
    user_uuid: &Uuid,
) -> anyhow::Result<(), Errors>
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

    let hashtag_names = if let Some(ref tags) = hashtags {
        if !tags.is_empty() {
            repository_associate_post_hashtags(&txn, created_post.id, tags).await?;
        }
        tags.clone()
    } else {
        Vec::new()
    };

    // Commit the transaction
    txn.commit().await?;

    // Meilisearch 인덱싱 (DB 트랜잭션 외부에서 실행)
    if let Ok(user) = repository_find_user_by_uuid(conn, user_uuid).await {
        if let Some(user) = user {
            let meilisearch_post = MeilisearchPost::from_post_with_user_and_hashtags(
                &created_post,
                &user,
                hashtag_names,
            );

            post_indexer::index_post(meilisearch, &meilisearch_post).await;
        }
    }

    Ok(())
}
