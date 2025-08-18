use crate::entity::posts::{ActiveModel as PostActiveModel, Entity as PostEntity};
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, Set};
use uuid::Uuid;

pub async fn repository_increment_comment_count<C>(
    conn: &C,
    post_id: &Uuid,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let post = PostEntity::find_by_id(*post_id)
        .one(conn)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("Post not found".to_string()))?;

    let mut post_active: PostActiveModel = post.into();
    let current_count = post_active.comment_count.unwrap();
    post_active.comment_count = Set(current_count + 1);

    post_active.update(conn).await?;
    Ok(())
}

pub async fn repository_decrement_comment_count<C>(
    conn: &C,
    post_id: &Uuid,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let post = PostEntity::find_by_id(*post_id)
        .one(conn)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("Post not found".to_string()))?;

    let mut post_active: PostActiveModel = post.into();
    let current_count = post_active.comment_count.unwrap();
    post_active.comment_count = Set(std::cmp::max(0, current_count - 1));

    post_active.update(conn).await?;
    Ok(())
}