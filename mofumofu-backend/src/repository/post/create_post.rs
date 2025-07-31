use crate::dto::post::request::create::CreatePostRequest;
use crate::entity::posts::ActiveModel as PostActiveModel;
use crate::service::error::errors::Errors;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set, TransactionTrait};
use uuid::Uuid;

pub async fn repository_create_post<C>(
    txn: &C,
    payload: CreatePostRequest,
    user_uuid: &Uuid,
) -> Result<(), Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_post = PostActiveModel {
        id: Default::default(),
        title: Set(payload.title),
        summary: Set(payload.summary),
        user_id: Set(*user_uuid),
        content: Set(payload.content),
        created_at: Set(Utc::now()),
        updated_at: Set(Option::from(Utc::now())),
        is_deleted: Set(false),
        deleted_at: Default::default(),
        like_count: Set(0),
        comment_count: Set(0),
        view_count: Set(0),
        slug: Set(payload.slug),
    };

    // Insert the new post
    new_post.insert(txn).await?;

    Ok(())
}
