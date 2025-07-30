use uuid::Uuid;
use crate::entity::common::OAuthProvider;

pub struct OAuthConnectionResponse {
    pub id: Uuid,
    pub provider: OAuthProvider,
    pub provider_user_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}