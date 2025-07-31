use crate::config::db_config::DbConfig;
use crate::service::error::errors::Errors;
use crate::service::oauth::config::build_oauth_client;

pub fn build_google_client() -> Result<impl Clone, Errors> {
    let config = &DbConfig::get();
    build_oauth_client(
        &config.google_client_id,
        &config.google_client_secret,
        &config.google_redirect_uri,
        "https://accounts.google.com/o/oauth2/v2/auth",
        "https://oauth2.googleapis.com/token",
    )
}