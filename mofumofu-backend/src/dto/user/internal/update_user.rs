#[derive(Default)]
pub struct UpdateUserFields {
    pub name: Option<String>,
    pub handle: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub is_verified: Option<bool>,
    pub profile_image: Option<Option<String>>, // Option<String>을 Option으로 감싸서 NULL 설정도 가능
    pub banner_image: Option<Option<String>>,
}