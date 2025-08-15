use crate::entity::likes::{
    ActiveModel as LikesActiveModel, Column as LikesColumn, Entity as LikesEntity, Relation as LikesRelation,
};
use crate::entity::posts::{Column as PostColumn, Entity as PostEntity, Relation as PostRelation};
use crate::entity::users::{Column as UserColumn, Entity as UserEntity};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait};
use uuid::Uuid;

pub async fn repository_delete_like_by_handle_and_slug<C>(
    conn: &C,
    user_id: Uuid,
    handle: &str,
    slug: &str,
) -> Result<bool, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    // handle과 slug로 like 찾기
    let existing_like = LikesEntity::find()
        .join(JoinType::InnerJoin, LikesRelation::Post.def())
        .join(JoinType::InnerJoin, PostRelation::User.def())
        .filter(LikesColumn::UserId.eq(user_id))
        .filter(PostColumn::Slug.eq(slug))
        .filter(UserColumn::Handle.eq(handle))
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