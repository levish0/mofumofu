use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct DeleteLikeRequest {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Handle must be between 3 and 20 characters."
    ))]
    pub handle: String,
    #[validate(length(
        min = 1,
        max = 255,
        message = "Slug must be between 1 and 255 characters."
    ))]
    pub slug: String,
}