use crate::entity::likes::{ActiveModel as LikesActiveModel, Model as LikesModel};
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

pub async fn repository_create_like<C>(
    conn: &C,
    user_id: Uuid,
    post_id: Uuid,
) -> Result<LikesModel, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let new_like = LikesActiveModel {
        id: Default::default(),
        user_id: Set(user_id),
        post_id: Set(post_id),
        created_at: Default::default(),
    };

    let created_like = new_like.insert(conn).await?;
    Ok(created_like)
}