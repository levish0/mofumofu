use serde::{Deserialize, Serialize};

pub struct OAuthUserData {
    pub provider_user_id: String,
    pub email: String,
    pub name: String,          // 닉네임/이름
    pub profile_image: String, // 프로필 이미지 URL
}