use std::time::Duration;
use crate::config::db_config::DbConfig;
use crate::dto::oauth::internal::google::GoogleUserInfo;
use crate::service::error::errors::Errors;
use crate::service::oauth::provider::common::{build_oauth_client, exchange_oauth_code};
use oauth2::basic::{
    BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
    BasicTokenResponse,
};
use oauth2::{AccessToken, Client as OauthClient, EndpointNotSet, EndpointSet, StandardRevocableToken};
use reqwest::{Client as ReqwestClient, };
use sea_orm::Iden;
use tracing::{error, info, warn};
use crate::connection::cloudflare_r2::R2Client;

fn build_google_client() -> Result<
    OauthClient<
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
        &config.google_client_id,
        &config.google_client_secret,
        &config.google_redirect_uri,
        "https://accounts.google.com/o/oauth2/v2/auth",
        "https://oauth2.googleapis.com/token",
    )
}

pub async fn exchange_google_code(code: &str) -> Result<AccessToken, Errors> {
    let client = build_google_client()?;
    exchange_oauth_code(client, code, "Google").await
}

pub async fn get_google_user_info(http_client: &ReqwestClient, r2_client: &R2Client,access_token: &AccessToken) -> Result<GoogleUserInfo, Errors> {
    let response = http_client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(access_token.secret())
        .send()
        .await
        .map_err(|e| {
            error!("Failed to fetch Google user info: {:?}", e);
            Errors::OauthUserInfoFetchFailed
        })?;

    if !response.status().is_success() {
        error!("Google API returned error status: {}", response.status());
        return Err(Errors::OauthUserInfoFetchFailed);
    }

    let user_info: GoogleUserInfo = response.json().await.map_err(|e| {
        error!("Failed to parse Google user info: {:?}", e);
        Errors::OauthUserInfoParseFailed
    })?;
    

    Ok(user_info)
}