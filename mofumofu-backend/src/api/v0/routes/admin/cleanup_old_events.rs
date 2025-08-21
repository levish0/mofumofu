use axum::{extract::State, http::StatusCode, Extension};


use crate::{
    dto::admin::response::AdminTaskResponse,
    dto::auth::internal::access_token::AccessTokenClaims,
    service::admin::cleanup_old_events::service_cleanup_old_events,
    state::AppState,
};


/// 오래된 시스템 이벤트 정리
#[utoipa::path(
    post,
    path = "/v0/admin/cleanup/events",
    summary = "오래된 시스템 이벤트 정리",
    description = "30일 이상 된 시스템 이벤트를 데이터베이스에서 정리합니다. (Admin 전용)",
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
pub async fn cleanup_old_events(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<AccessTokenClaims>,
) -> Result<AdminTaskResponse, StatusCode> {
    match service_cleanup_old_events(&app_state, token_data.sub).await {
        Ok(data) => Ok(AdminTaskResponse {
            success: true,
            message: "오래된 시스템 이벤트 정리 작업이 시작되었습니다".to_string(),
            data: Some(data),
        }),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}