use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PostInfoResponse {
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
    pub author: PostAuthor,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub like_count: i32,
    pub comment_count: i32,
    pub view_count: i32,
    pub slug: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PostAuthor {
    pub handle: String,
    pub name: String,
    pub profile_image: Option<String>,
}

impl IntoResponse for PostInfoResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PostSummary {
    pub id: Uuid,
    pub title: String,
    pub summary: Option<String>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub like_count: i32,
    pub comment_count: i32,
    pub view_count: i32,
    pub slug: Option<String>,
}
