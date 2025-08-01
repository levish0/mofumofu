use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CreatePostRequest {
    #[validate(length(max = 200, message = "Title must be at most 200 characters."))]
    pub title: String,
    #[validate(length(max = 500, message = "Summary must be at most 500 characters."))]
    pub summary: Option<String>,
    #[validate(length(max = 40000, message = "Content must be at most 40000 characters."))]
    pub content: String,
    #[validate(length(max = 255, message = "Slug must be at most 255 characters."))]
    pub slug: Option<String>,
}
