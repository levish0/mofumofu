use crate::dto::post::internal::create::CreatePost;
use crate::entity::posts::ActiveModel as PostActiveModel;
use crate::service::error::errors::Errors;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection, Set, TransactionTrait};

pub async fn service_create_post<C>(conn: &C, payload: CreatePost) -> anyhow::Result<(), Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    let new_post = PostActiveModel {
        id: Default::default(),
        title: Default::default(),
        user_id: Set(payload.author_id),
        content: Set(payload.content),
        created_at: Set(Utc::now()),
        updated_at: Set(Option::from(Utc::now())),
        is_deleted: Default::default(),
        status: Default::default(),
        published_at: Default::default(),
        last_auto_saved_at: Default::default(),
        like_count: Default::default(),
        comment_count: Default::default(),
        view_count: Default::default(),
        summary: Default::default(),
        slug: Default::default(),
    };

    // Insert the new post
    new_post.insert(&txn).await?;

    // Commit the transaction
    txn.commit().await?;

    Ok(())
}
