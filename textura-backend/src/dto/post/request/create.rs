use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CreatePostRequest {
    #[validate(length(max = 2000, message = "Content must be at most 2000 characters."))]
    pub content: String,
}
