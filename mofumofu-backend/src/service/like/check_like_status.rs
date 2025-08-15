use crate::dto::like::response::like_status::LikeStatusResponse;
use crate::repository::like::check_like_status::repository_check_like_status;
use crate::service::error::errors::ServiceResult;
use sea_orm::ConnectionTrait;
use uuid::Uuid;

pub async fn service_check_like_status<C>(
    conn: &C,
    user_id: &Uuid,
    handle: &str,
    slug: &str,
) -> ServiceResult<LikeStatusResponse>
where
    C: ConnectionTrait,
{
    let is_liked = repository_check_like_status(conn, user_id, handle, slug).await?;

    Ok(LikeStatusResponse { is_liked })
}