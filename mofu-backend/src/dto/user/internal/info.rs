use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UserInfo {
    pub id: Uuid,
    pub name: String,
    pub handle: String,
    pub email: String,
    pub profile_image: Option<String>,
    pub banner_image: Option<String>,
}
