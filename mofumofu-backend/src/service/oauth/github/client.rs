use crate::config::db_config::DbConfig;
use crate::service::error::errors::Errors;
use crate::service::oauth::config::build_oauth_client;

pub fn build_github_client() -> Result<impl Clone, Errors> {
    let config = &DbConfig::get();
    build_oauth_client(
        &config.github_client_id,
        &config.github_client_secret,
        &config.github_redirect_uri,
        "https://github.com/login/oauth/authorize",
        "https://github.com/login/oauth/access_token",
    )
}
