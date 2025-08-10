use crate::dto::post::request::PostSortOrder;
use crate::entity::posts::{Column, Entity as PostEntity, Model as PostModel};
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, PaginatorTrait, QuerySelect};

pub async fn repository_get_posts<C>(
    conn: &C,
    page: u32,
    page_size: u32,
    sort_order: &PostSortOrder,
) -> Result<Vec<PostModel>, Errors>
where
    C: ConnectionTrait,
{
    let offset = (page - 1) * page_size;

    let mut query = PostEntity::find();

    // 정렬 조건 적용
    match sort_order {
        PostSortOrder::Latest => {
            query = query.order_by_desc(Column::CreatedAt);
        }
        PostSortOrder::Popular => {
            // 좋아요 수 기준으로 정렬 (나중에 더 복잡한 로직으로 변경 가능)
            query = query.order_by_desc(Column::LikeCount);
        }
        PostSortOrder::Oldest => {
            query = query.order_by_asc(Column::CreatedAt);
        }
    }

    let posts = query
        .offset(offset as u64)
        .limit(page_size as u64)
        .all(conn)
        .await?;

    Ok(posts)
}

pub async fn repository_get_posts_around_page<C>(
    conn: &C,
    target_page: u32,
    page_size: u32,
    pages_around: u32,
    sort_order: &PostSortOrder,
) -> Result<Vec<PostModel>, Errors>
where
    C: ConnectionTrait,
{
    let start_page = if target_page > pages_around {
        target_page - pages_around
    } else {
        1
    };
    let end_page = target_page + pages_around;
    
    let start_offset = (start_page - 1) * page_size;
    let total_items = (end_page - start_page + 1) * page_size;

    let mut query = PostEntity::find();

    match sort_order {
        PostSortOrder::Latest => {
            query = query.order_by_desc(Column::CreatedAt);
        }
        PostSortOrder::Popular => {
            query = query.order_by_desc(Column::LikeCount);
        }
        PostSortOrder::Oldest => {
            query = query.order_by_asc(Column::CreatedAt);
        }
    }

    let posts = query
        .offset(start_offset as u64)
        .limit(total_items as u64)
        .all(conn)
        .await?;

    Ok(posts)
}

pub async fn repository_get_posts_count<C>(conn: &C) -> Result<u64, Errors>
where
    C: ConnectionTrait,
{
    let count = PostEntity::find()
        .count(conn)
        .await?;

    Ok(count)
}