use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub struct GetCommentsRequest {
    pub post_id: Uuid,
    
    #[serde(default = "default_page")]
    pub page: u32,
    
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub struct GetRepliesRequest {
    pub parent_comment_id: Uuid,
    
    #[serde(default = "default_page")]
    pub page: u32,
    
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 {
    1
}

fn default_per_page() -> u32 {
    20
}