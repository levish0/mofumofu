use crate::dto::post::internal::create::CreatePost;
use crate::dto::post::request::create::CreatePostRequest;
use crate::entity::posts::ActiveModel as PostActiveModel;
use crate::service::error::errors::Errors;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TransactionTrait};

pub async fn service_create_post(
    conn: &DatabaseConnection,
    payload: CreatePost,
) -> anyhow::Result<(), Errors> {
    let txn = conn.begin().await?;

    let new_post = PostActiveModel {
        id: Default::default(),
        author_id: Set(payload.author_id),
        content: Set(payload.content),
        created_at: Set(Utc::now()),
        updated_at: Set(Option::from(Utc::now())),
        like_count: Default::default(),
        reply_count: Default::default(),
    };

    // Insert the new post
    new_post.insert(&txn).await?;

    // Commit the transaction
    txn.commit().await?;

    Ok(())
}
