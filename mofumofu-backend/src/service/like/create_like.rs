use crate::entity::common::{ActionType, TargetType};
use crate::entity::likes::{Column as LikesColumn, Entity as LikesEntity};
use crate::repository::like::create_like::repository_create_like;
use crate::repository::post::get_post_by_uuid::repository_get_post_by_uuid;
use crate::repository::system_events::log_event::repository_log_event;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, TransactionTrait};
use uuid::Uuid;

pub async fn service_create_like<C>(
    conn: &C,
    user_id: &Uuid,
    post_id: &Uuid,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    // 포스트 존재 확인
    let post = repository_get_post_by_uuid(&txn, post_id).await?;

    // 자신의 포스트에는 좋아요를 누를 수 없음
    if post.user_id == *user_id {
        return Err(Errors::BadRequestError("Cannot like your own post".to_string()));
    }

    // 이미 좋아요가 있는지 확인
    let existing_like = LikesEntity::find()
        .filter(LikesColumn::UserId.eq(*user_id))
        .filter(LikesColumn::PostId.eq(*post_id))
        .one(&txn)
        .await?;

    if existing_like.is_some() {
        return Err(Errors::BadRequestError("Already liked this post".to_string()));
    }

    // 좋아요 생성
    let _created_like = repository_create_like(&txn, *user_id, *post_id).await?;

    txn.commit().await?;

    // 좋아요 생성 이벤트 로깅
    repository_log_event(
        conn,
        Some(*user_id),
        ActionType::LikeCreated,
        Some(*post_id),
        Some(TargetType::Post),
        None,
    )
    .await;

    Ok(())
}