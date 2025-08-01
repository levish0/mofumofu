use crate::config::db_config::DbConfig;
use crate::service::error::errors::Errors;
use oauth2::basic::{
    BasicClient, BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse,
};
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, EndpointNotSet,
    EndpointSet, RedirectUrl, StandardRevocableToken, TokenResponse, TokenUrl,
};
use tracing::error;

pub fn build_oauth_client(
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
    auth_url: &str,
    token_url: &str,
) -> Result<
    Client<
        BasicErrorResponse,
        BasicTokenResponse,
        BasicTokenIntrospectionResponse,
        StandardRevocableToken,
        BasicRevocationErrorResponse,
        EndpointSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointSet,
    >,
    Errors,
> {
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

pub fn create_http_client() -> Result<reqwest::Client, Errors> {
    reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none()) // SSRF 방지
        .build()
        .map_err(|e| Errors::OauthTokenExchangeFailed)
}

// 공통 토큰 교환 함수
pub async fn exchange_oauth_code(
    oauth_client: Client<
        BasicErrorResponse,
        BasicTokenResponse,
        BasicTokenIntrospectionResponse,
        StandardRevocableToken,
        BasicRevocationErrorResponse,
        EndpointSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointSet,
    >,
    code: &str,
    provider: &str,
) -> Result<AccessToken, Errors> {
    let http_client = create_http_client()?;

    let token_result = oauth_client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request_async(&http_client)
        .await
        .map_err(|e| {
            error!("Failed to exchange {} code: {:?}", provider, e);
            Errors::OauthTokenExchangeFailed
        })?;

    Ok(token_result.access_token().clone())
}
