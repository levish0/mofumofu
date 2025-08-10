use crate::connection::cloudflare_r2::R2Client;
use crate::connection::meilisearch::MeilisearchClient;
use reqwest::Client;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub cloudflare_r2: R2Client,
    pub http_client: Client,
    pub meilisearch: MeilisearchClient,
}
