use crate::dto::user::request::create::CreateUserRequest;
use crate::entity::common::{ActionType, TargetType};
use crate::repository::system_events::log_event::repository_log_event;
use crate::repository::user::create_user::repository_create_user;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, DatabaseConnection, TransactionTrait};

pub async fn service_create_user<C>(conn: &C, payload: CreateUserRequest) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    let user = repository_create_user(&txn, payload).await?;

    txn.commit().await?;

    // 이벤트 로깅 - 별도 실행, 실패해도 메인 로직에 영향 없음
    repository_log_event(
        conn,
        Some(user.id),
        ActionType::UserCreated,
        Some(user.id),
        Some(TargetType::User),
        None,
    )
    .await;

    Ok(())
}
