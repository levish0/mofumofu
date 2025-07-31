use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoogleLoginRequest {
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubLoginRequest {
    pub code: String,
}
