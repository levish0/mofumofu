use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
#[derive(Debug)]
pub struct CreateCommentLikeRequest {
    pub comment_id: Uuid,
}