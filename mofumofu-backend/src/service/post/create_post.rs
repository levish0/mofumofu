use crate::dto::post::request::create::CreatePostRequest;
use crate::repository::post::create_post::repository_create_post;
use crate::service::error::errors::Errors;
use sea_orm::{ConnectionTrait, TransactionTrait};
use uuid::Uuid;

pub async fn service_create_post<C>(
    conn: &C,
    payload: CreatePostRequest,
    user_uuid: &Uuid,
) -> anyhow::Result<(), Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    repository_create_post(&txn, payload, user_uuid).await?;

    // Commit the transaction
    txn.commit().await?;

    Ok(())
}
