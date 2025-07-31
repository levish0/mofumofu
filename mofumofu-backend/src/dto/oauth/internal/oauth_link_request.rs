use crate::entity::common::OAuthProvider;

pub struct OAuthLinkRequest {
    pub provider: OAuthProvider,
    pub code: String,
    pub state: Option<String>,
}
