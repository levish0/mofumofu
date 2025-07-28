use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CreatePost {
    #[validate(length(max = 40000, message = "Content must be at most 40000 characters."))]
    pub content: String,
    pub author_id: Uuid,
    pub reply_to_id: Option<Uuid>,
}
