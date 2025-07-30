use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CreatePostRequest {
    #[validate(length(max = 2000, message = "Content must be at most 2000 characters."))]
    pub content: String,
    pub reply_to_id: Option<Uuid>,
}
