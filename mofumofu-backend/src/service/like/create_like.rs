use crate::entity::common::{ActionType, TargetType};
use crate::repository::like::check_like_status::repository_check_like_status;
use crate::repository::like::create_like::repository_create_like_by_handle_and_slug;
use crate::repository::post::find_post_by_handle_and_slug::repository_find_post_by_handle_and_slug;
use crate::repository::post::update_like_count::repository_increment_post_like_count;
use crate::repository::system_events::log_event::repository_log_event;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionTrait};
use uuid::Uuid;

pub async fn service_create_like<C>(
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

    // 자신의 포스트에는 좋아요를 누를 수 없음
    if post.user_id == *user_id {
        return Err(Errors::BadRequestError("Cannot like your own post".to_string()));
    }

    // 이미 좋아요가 있는지 확인
    let already_liked = repository_check_like_status(&txn, user_id, handle, slug).await?;
    if already_liked {
        return Err(Errors::BadRequestError("Already liked this post".to_string()));
    }

    // 좋아요 생성
    let _created_like = repository_create_like_by_handle_and_slug(&txn, *user_id, handle, slug).await?;

    // 포스트 좋아요 개수 증가
    repository_increment_post_like_count(&txn, post.id).await?;

    txn.commit().await?;

    // 좋아요 생성 이벤트 로깅
    repository_log_event(
        conn,
        Some(*user_id),
        ActionType::LikeCreated,
        Some(post.id),
        Some(TargetType::Post),
        None,
    )
    .await;

    Ok(())
}