use crate::entity::likes::{
    ActiveModel as LikesActiveModel, Column as LikesColumn, Entity as LikesEntity,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_delete_like<C>(
    conn: &C,
    user_id: Uuid,
    post_id: Uuid,
) -> Result<bool, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let existing_like = LikesEntity::find()
        .filter(LikesColumn::UserId.eq(user_id))
        .filter(LikesColumn::PostId.eq(post_id))
        .one(conn)
        .await?;

    match existing_like {
        Some(like_record) => {
            let like_active_model: LikesActiveModel = like_record.into();
            like_active_model.delete(conn).await?;
            Ok(true)
        }
        None => Ok(false),
    }
}