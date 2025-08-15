use crate::entity::likes::{ActiveModel as LikesActiveModel, Model as LikesModel};
use crate::entity::posts::{Column as PostColumn, Entity as PostEntity, Relation as PostRelation};
use crate::entity::users::{Column as UserColumn, Entity as UserEntity};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait, Set};
use uuid::Uuid;

pub async fn repository_create_like_by_handle_and_slug<C>(
    conn: &C,
    user_id: Uuid,
    handle: &str,
    slug: &str,
) -> Result<LikesModel, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    // handle과 slug로 post 찾기
    let post = PostEntity::find()
        .join(JoinType::InnerJoin, PostRelation::User.def())
        .filter(PostColumn::Slug.eq(slug))
        .filter(UserColumn::Handle.eq(handle))
        .one(conn)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("Post not found".to_string()))?;

    let new_like = LikesActiveModel {
        id: Default::default(),
        user_id: Set(user_id),
        post_id: Set(post.id),
        created_at: Default::default(),
    };

    let created_like = new_like.insert(conn).await?;
    Ok(created_like)
}