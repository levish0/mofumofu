use crate::config::db_config::DbConfig;
use crate::service::error::errors::Errors;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

pub fn build_google_client() -> Result<impl Clone, Errors> {
    let client_id = &DbConfig::get().google_client_id;
    let client_secret = &DbConfig::get().google_client_secret;
    let redirect_uri = &DbConfig::get().google_redirect_uri;

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .map_err(|_| Errors::SysOauthProviderNotSupported)?;
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
        .map_err(|_| Errors::SysOauthProviderNotSupported)?;
    let redirect_url = RedirectUrl::new(redirect_uri.clone())
        .map_err(|_| Errors::SysOauthProviderNotSupported)?;

    let client = BasicClient::new(ClientId::new(client_id.clone()))
        .set_client_secret(ClientSecret::new(client_secret.clone()))
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url);

    Ok(client)
}

pub fn build_github_client() -> Result<impl Clone, Errors> {
    let client_id = &DbConfig::get().github_client_id;
    let client_secret = &DbConfig::get().github_client_secret;
    let redirect_uri = &DbConfig::get().github_redirect_uri;

    let client = BasicClient::new(ClientId::new(client_id.clone()))
        .set_client_secret(ClientSecret::new(client_secret.clone()))
        .set_auth_uri(
            AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                .map_err(|_| Errors::SysOauthProviderNotSupported)?
        )
        .set_token_uri(
            TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                .map_err(|_| Errors::SysOauthProviderNotSupported)?
        )
        .set_redirect_uri(
            RedirectUrl::new(redirect_uri.clone())
                .map_err(|_| Errors::SysOauthProviderNotSupported)?
        );

    Ok(client)
}