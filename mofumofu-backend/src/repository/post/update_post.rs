use crate::dto::post::request::update_post::UpdatePostRequest;
use crate::entity::posts::{ActiveModel as PostActiveModel, Column, Entity as PostEntity, Model as PostModel};
use crate::service::error::errors::Errors;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use uuid::Uuid;

pub async fn repository_update_post<C>(
    conn: &C,
    payload: UpdatePostRequest,
    user_uuid: &Uuid,
) -> Result<PostModel, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let post = PostEntity::find()
        .filter(Column::Slug.eq(&payload.slug))
        .filter(Column::UserId.eq(*user_uuid))
        .one(conn)
        .await?
        .ok_or(Errors::PostNotFound)?;

    let mut active_post: PostActiveModel = post.into();

    if let Some(title) = payload.title {
        active_post.title = Set(title);
    }
    
    if let Some(summary) = payload.summary {
        active_post.summary = Set(summary);
    }
    
    if let Some(content) = payload.content {
        active_post.content = Set(content);
    }
    
    if let Some(new_slug) = payload.new_slug {
        active_post.slug = Set(new_slug);
    }

    active_post.updated_at = Set(Some(Utc::now()));

    let updated_post = active_post.update(conn).await?;

    Ok(updated_post)
}