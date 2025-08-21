use axum::{extract::State, http::StatusCode, Extension};

use crate::{
    dto::admin::response::AdminTaskResponse,
    dto::auth::internal::access_token::AccessTokenClaims,
    service::admin::meilisearch_health::service_meilisearch_health,
    state::AppState,
};

/// Meilisearch 헬스체크
#[utoipa::path(
    post,
    path = "/v0/admin/search/health",
    summary = "Meilisearch 헬스체크",
    description = "Meilisearch 서버 상태를 확인합니다. (Admin 전용)",
    responses(
        (status = 200, description = "헬스체크 완료", body = AdminTaskResponse),
        (status = 401, description = "인증 실패"),
        (status = 403, description = "Admin 권한 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"
)]
pub async fn meilisearch_health(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<AccessTokenClaims>,
) -> Result<AdminTaskResponse, StatusCode> {
    match service_meilisearch_health(&app_state, token_data.sub).await {
        Ok(data) => Ok(AdminTaskResponse {
            success: true,
            message: "Meilisearch 헬스체크가 완료되었습니다".to_string(),
            data: Some(data),
        }),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}