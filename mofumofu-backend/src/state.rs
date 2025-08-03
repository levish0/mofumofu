use reqwest::Client;
use sea_orm::DatabaseConnection;
use crate::connection::cloudflare_r2::R2Client;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub cloudflare_r2: R2Client,
    pub http_client: Client
}
