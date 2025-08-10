use crate::entity::hash_tags::{Entity as HashTagEntity, Model as HashTagModel, Column};
use crate::service::error::errors::Errors;
use sea_orm::{ConnectionTrait, EntityTrait, QueryOrder, QuerySelect};

pub async fn repository_get_popular_hashtags<C>(
    conn: &C,
    limit: u64,
) -> Result<Vec<HashTagModel>, Errors>
where
    C: ConnectionTrait,
{
    let hashtags = HashTagEntity::find()
        .order_by_desc(Column::UsageCount)
        .limit(limit)
        .all(conn)
        .await?;

    Ok(hashtags)
}

pub async fn repository_get_recent_hashtags<C>(
    conn: &C,
    limit: u64,
) -> Result<Vec<HashTagModel>, Errors>
where
    C: ConnectionTrait,
{
    let hashtags = HashTagEntity::find()
        .order_by_desc(Column::LastUsedAt)
        .limit(limit)
        .all(conn)
        .await?;

    Ok(hashtags)
}