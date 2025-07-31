use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct GoogleLoginRequest {
    #[validate(length(min = 1, message = "Authorization code is required"))]
    pub code: String,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct GithubLoginRequest {
    #[validate(length(min = 1, message = "Authorization code is required"))]
    pub code: String,
}
