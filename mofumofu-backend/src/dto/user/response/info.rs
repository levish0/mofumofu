use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UserInfoResponse {
    pub name: String,
    pub handle: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<String>,
}

impl IntoResponse for UserInfoResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
