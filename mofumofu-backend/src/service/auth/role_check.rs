use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::entity::common::UserRole;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::ConnectionTrait;
use uuid::Uuid;

pub async fn require_moderator<C>(
    conn: &C,
    claims: &AccessTokenClaims,
) -> ServiceResult<()>
where
    C: ConnectionTrait,
{
    let user = repository_find_user_by_uuid(conn, &claims.sub)
        .await?
        .ok_or(Errors::UserNotFound)?;

    match user.role {
        UserRole::Moderator | UserRole::Admin => Ok(()),
        _ => Err(Errors::UserUnauthorized),
    }
}

pub async fn require_admin<C>(
    conn: &C,
    claims: &AccessTokenClaims,
) -> ServiceResult<()>
where
    C: ConnectionTrait,
{
    let user = repository_find_user_by_uuid(conn, &claims.sub)
        .await?
        .ok_or(Errors::UserNotFound)?;

    match user.role {
        UserRole::Admin => Ok(()),
        _ => Err(Errors::UserUnauthorized),
    }
}

pub async fn get_user_role<C>(
    conn: &C,
    claims: &AccessTokenClaims,
) -> ServiceResult<UserRole>
where
    C: ConnectionTrait,
{
    let user = repository_find_user_by_uuid(conn, &claims.sub)
        .await?
        .ok_or(Errors::UserNotFound)?;

    Ok(user.role)
}