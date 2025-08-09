use crate::entity::posts::{Column, Entity as UserEntity, Model as PostModel};
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

pub async fn repository_find_post_by_slug<C>(
    conn: &C,
    slug: &str,
) -> Result<Option<PostModel>, Errors>
where
    C: ConnectionTrait,
{
    Ok(UserEntity::find()
        .filter(Column::Slug.eq(slug))
        .one(conn)
        .await?)
}
