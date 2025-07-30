use serde::Deserialize;

// OAuth 로그인 요청
pub struct OAuthLoginRequest {
    pub provider: String,
    pub code: String,
    pub state: Option<String>,
}