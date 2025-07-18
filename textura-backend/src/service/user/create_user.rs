use crate::dto::user::request::create::CreateUserRequest;
use crate::entity::users::ActiveModel as UserActiveModel;
use crate::service::error::errors::Errors;
use crate::utils::crypto::hash_password;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TransactionTrait};

pub async fn service_create_user(
    conn: &DatabaseConnection,
    payload: CreateUserRequest,
) -> Result<(), Errors> {
    let hashed_password = hash_password(&payload.password)?;

    let txn = conn.begin().await?;

    let new_user = UserActiveModel {
        id: Default::default(),
        name: Set(payload.name),
        handle: Set(payload.handle),
        email: Set(payload.email),
        password: Set(hashed_password),
        profile_image: Default::default(),
        banner_image: Default::default(),
    };

    new_user.insert(&txn).await?;

    txn.commit().await?;

    Ok(())
}
