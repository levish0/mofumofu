use crate::entity::posts::{Column, Entity as UserEntity, Model as PostModel};
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use crate::repository::post::find_post_by_slug::repository_find_post_by_slug;

pub async fn repository_get_post_by_slug<C>(
    conn: &C,
    slug: &str,
) -> Result<PostModel, Errors>
where
    C: ConnectionTrait,
{
    repository_find_post_by_slug(conn, slug).await?.ok_or(Errors::PostNotFound)
}
