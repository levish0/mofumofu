use axum::{extract::State, http::StatusCode, Extension};


use crate::{
    dto::admin::response::AdminTaskResponse,
    dto::auth::internal::access_token::AccessTokenClaims,
    service::admin::search_stats::service_search_stats,
    state::AppState,
};


/// 검색 색인 통계 조회
#[utoipa::path(
    post,
    path = "/v0/admin/search/stats",
    summary = "검색 색인 통계 조회",
    description = "Meilisearch 색인 통계를 조회합니다. (Admin 전용)",
    responses(
        (status = 200, description = "통계 조회 완료", body = AdminTaskResponse),
        (status = 401, description = "인증 실패"),
        (status = 403, description = "Admin 권한 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"
)]
pub async fn search_stats(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<AccessTokenClaims>,
) -> Result<AdminTaskResponse, StatusCode> {
    match service_search_stats(&app_state, token_data.sub).await {
        Ok(data) => Ok(AdminTaskResponse {
            success: true,
            message: "검색 색인 통계 조회가 완료되었습니다".to_string(),
            data: Some(data),
        }),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}