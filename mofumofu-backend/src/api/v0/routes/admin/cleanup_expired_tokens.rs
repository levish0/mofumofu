use axum::{extract::State, http::StatusCode, Extension};


use crate::{
    dto::admin::response::AdminTaskResponse,
    dto::auth::internal::access_token::AccessTokenClaims,
    service::admin::cleanup_expired_tokens::service_cleanup_expired_tokens,
    state::AppState,
};


/// 만료된 토큰 정리
#[utoipa::path(
    post,
    path = "/v0/admin/cleanup/tokens",
    summary = "만료된 리프레시 토큰 정리",
    description = "만료되거나 폐기된 리프레시 토큰을 데이터베이스에서 정리합니다. (Admin 전용)",
    responses(
        (status = 200, description = "정리 작업 시작됨", body = AdminTaskResponse),
        (status = 401, description = "인증 실패"),
        (status = 403, description = "Admin 권한 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"
)]
pub async fn cleanup_expired_tokens(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<AccessTokenClaims>,
) -> Result<AdminTaskResponse, StatusCode> {
    match service_cleanup_expired_tokens(&app_state, token_data.sub).await {
        Ok(data) => Ok(AdminTaskResponse {
            success: true,
            message: "만료된 토큰 정리 작업이 시작되었습니다".to_string(),
            data: Some(data),
        }),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}