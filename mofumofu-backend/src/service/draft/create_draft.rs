use crate::dto::draft::request::create_draft::CreateDraftRequest;
use crate::dto::draft::response::draft_info::DraftInfo;
use crate::repository::draft::create_draft::repository_create_draft;
use crate::repository::draft::get_draft_count::repository_get_draft_count;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, DbErr, TransactionTrait};
use tracing::info;
use uuid::Uuid;

pub async fn service_create_draft<C>(
    conn: &C,
    payload: CreateDraftRequest,
    user_uuid: &Uuid,
) -> ServiceResult<DraftInfo>
where
    C: ConnectionTrait + TransactionTrait,
{
    // 드래프트 개수 체크 (10개 제한)
    let current_count = repository_get_draft_count(conn, user_uuid).await?;
    if current_count >= 10 {
        return Err(Errors::DraftLimitExceeded);
    }

    let txn = conn.begin().await?;

    let created_draft = match repository_create_draft(&txn, payload, user_uuid).await {
        Ok(draft) => draft,
        Err(Errors::DatabaseError(msg)) if msg.contains("duplicate") || msg.contains("unique") => {
            return Err(Errors::DraftSlugAlreadyExists);
        }
        Err(e) => return Err(e),
    };

    txn.commit().await?;

    info!("드래프트 생성 완료 (draft_id: {})", created_draft.id);

    Ok(DraftInfo {
        draft_id: created_draft.id,
        title: created_draft.title,
        thumbnail_image: created_draft.thumbnail_image,
        summary: created_draft.summary,
        content: created_draft.content,
        slug: created_draft.slug,
        created_at: created_draft.created_at,
        updated_at: created_draft.updated_at,
    })
}
