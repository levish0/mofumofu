use crate::config::db_config::DbConfig;
use crate::service::error::errors::Errors;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

pub fn build_oauth_client(
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
    auth_url: &str,
    token_url: &str,
) -> Result<impl Clone, Errors> {
    let auth_url = AuthUrl::new(auth_url.to_string()).map_err(|_| Errors::OauthInvalidAuthUrl)?;
    let token_url =
        TokenUrl::new(token_url.to_string()).map_err(|_| Errors::OauthInvalidTokenUrl)?;
    let redirect_url =
        RedirectUrl::new(redirect_uri.to_string()).map_err(|_| Errors::OauthInvalidRedirectUrl)?;

    let client = BasicClient::new(ClientId::new(client_id.to_string()))
        .set_client_secret(ClientSecret::new(client_secret.to_string()))
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url);

    Ok(client)
}
