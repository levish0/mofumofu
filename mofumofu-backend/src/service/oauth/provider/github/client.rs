use crate::config::db_config::DbConfig;
use crate::dto::oauth::internal::github::{GithubEmail, GithubUserInfo};
use crate::dto::oauth::internal::google::GoogleUserInfo;
use crate::service::error::errors::Errors;
use crate::service::oauth::provider::common::{build_oauth_client, exchange_oauth_code};
use oauth2::basic::{
    BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse,
};
use oauth2::{AccessToken, Client, EndpointNotSet, EndpointSet, StandardRevocableToken};
use tracing::error;

fn build_github_client() -> Result<
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
    let config = &DbConfig::get();
    build_oauth_client(
        &config.github_client_id,
        &config.github_client_secret,
        &config.github_redirect_uri,
        "https://github.com/login/oauth/authorize",
        "https://github.com/login/oauth/access_token",
    )
}

pub async fn exchange_github_code(code: &str) -> Result<AccessToken, Errors> {
    let client = build_github_client()?;
    exchange_oauth_code(client, code, "GitHub").await
}

pub async fn get_github_user_info(access_token: &AccessToken) -> Result<GithubUserInfo, Errors> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token.secret())
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .map_err(|e| {
            error!("Failed to fetch GitHub user info: {:?}", e);
            Errors::OauthUserInfoFetchFailed
        })?;

    let mut user_info: GithubUserInfo = response.json().await.map_err(|e| {
        error!("Failed to parse Google user info: {:?}", e);
        Errors::OauthUserInfoParseFailed
    })?;

    if user_info.email.is_none() {
        let email_response = client
            .get("https://api.github.com/user/emails")
            .bearer_auth(access_token.secret())
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await;

        if let Ok(response) = email_response {
            if response.status().is_success() {
                if let Ok(emails) = response.json::<Vec<GithubEmail>>().await {
                    // primary 이메일 찾기
                    if let Some(primary_email) = emails.iter().find(|e| e.primary && e.verified) {
                        user_info.email = Some(primary_email.email.clone());
                    }
                }
            }
        }
    }

    Ok(user_info)
}
