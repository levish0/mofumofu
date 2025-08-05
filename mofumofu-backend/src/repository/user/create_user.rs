use crate::dto::user::request::create::CreateUserRequest;
use crate::entity::users::ActiveModel as UserActiveModel;
use crate::service::error::errors::Errors;
use crate::utils::crypto::hash_password;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set, TransactionTrait};

pub async fn repository_create_user<C>(txn: &C, payload: CreateUserRequest) -> Result<(), Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let hashed_password = hash_password(&payload.password)?;

    let new_user = UserActiveModel {
        id: Default::default(),
        name: Set(payload.name),
        handle: Set(payload.handle),
        bio: Default::default(),
        email: Set(payload.email),
        password: Set(Some(hashed_password)),
        is_verified: Set(false),
        profile_image: Default::default(),
        banner_image: Default::default(),
    };

    new_user.insert(txn).await?;

    Ok(())
}
