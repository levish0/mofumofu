use crate::dto::user::request::create::CreateUserRequest;
use crate::entity::users::ActiveModel as UserActiveModel;
use crate::service::error::errors::Errors;
use crate::utils::crypto::hash_password;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set, TransactionTrait};

pub async fn repository_create_oauth_user<C>(
    txn: &C,
    email: &str,
    name: &str,
    handle: &str,
    profile_image: Option<String>,
) -> Result<(), Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_user = UserActiveModel {
        id: Default::default(),
        name: Set(name.to_string()),
        handle: Set(handle.to_string()),
        email: Set(email.to_string()),
        password: Set(String::new()),
        is_verified: Set(true),
        profile_image: Set(profile_image),
        banner_image: Set(None),
    };

    new_user.insert(txn).await?;

    Ok(())
}
