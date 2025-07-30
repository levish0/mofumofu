use crate::dto::user::request::create::CreateUserRequest;
use crate::repository::user::create_user::repository_create_user;
use crate::service::error::errors::Errors;
use sea_orm::{ConnectionTrait, TransactionTrait};

pub async fn service_create_user<C>(conn: &C, payload: CreateUserRequest) -> Result<(), Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    repository_create_user(&txn, payload).await?;

    txn.commit().await?;

    Ok(())
}
