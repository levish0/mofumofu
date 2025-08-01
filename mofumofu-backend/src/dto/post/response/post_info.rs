use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PostInfo {
    pub id: Uuid,
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub is_deleted: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub like_count: i32,
    pub comment_count: i32,
    pub view_count: i32,
    pub slug: Option<String>,
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
