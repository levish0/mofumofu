use crate::repository::user::find_user_by_handle::repository_find_user_by_handle;
use crate::service::error::errors::Errors;
use crate::utils::generate_random_string::generate_random_string;
use rand::Rng;
use rand::distributions::Alphanumeric;
use sea_orm::ConnectionTrait;
use tracing::warn;
use uuid::Uuid;

pub async fn repository_generate_unique_handle<C>(conn: &C) -> Result<String, Errors>
where
    C: ConnectionTrait,
{
    const MAX_ATTEMPTS: u32 = 100;
    const HANDLE_LENGTH: usize = 10;

    for attempt in 0..MAX_ATTEMPTS {
        // UUID 기반 랜덤 핸들 생성
        let handle = generate_random_string(HANDLE_LENGTH);

        // 중복 확인
        if repository_find_user_by_handle(conn, &handle)
            .await?
            .is_none()
        {
            return Ok(handle);
        }

        warn!(
            "Handle collision detected on attempt {}: {}",
            attempt + 1,
            handle
        );
    }

    Err(Errors::UserHandleGenerationFailed)
}
