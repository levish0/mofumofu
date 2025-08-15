use crate::entity::common::{ActionType, TargetType};
use crate::repository::like::delete_like::repository_delete_like_by_handle_and_slug;
use crate::repository::post::find_post_by_handle_and_slug::repository_find_post_by_handle_and_slug;
use crate::repository::post::update_like_count::repository_decrement_post_like_count;
use crate::repository::system_events::log_event::repository_log_event;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionTrait};
use uuid::Uuid;

pub async fn service_delete_like<C>(
    conn: &C,
    user_id: &Uuid,
    handle: &str,
    slug: &str,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    // 포스트 존재 확인
    let post = repository_find_post_by_handle_and_slug(&txn, handle, slug)
        .await?
        .ok_or(Errors::PostNotFound)?;

    // 좋아요 삭제
    let deleted = repository_delete_like_by_handle_and_slug(&txn, *user_id, handle, slug).await?;

    if !deleted {
        return Err(Errors::BadRequestError("Like not found".to_string()));
    }

    // 포스트 좋아요 개수 감소
    repository_decrement_post_like_count(&txn, post.id).await?;

    txn.commit().await?;

    // 좋아요 삭제 이벤트 로깅
    repository_log_event(
        conn,
        Some(*user_id),
        ActionType::LikeDeleted,
        Some(post.id),
        Some(TargetType::Post),
        None,
    )
    .await;

    Ok(())
}