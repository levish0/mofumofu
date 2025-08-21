use axum::{extract::State, http::StatusCode, Extension};

use crate::{
    dto::admin::response::AdminTaskResponse,
    dto::auth::internal::access_token::AccessTokenClaims,
    service::admin::reindex_all_posts::service_reindex_all_posts,
    state::AppState,
};

/// 전체 포스트 재색인 트리거
#[utoipa::path(
    post,
    path = "/v0/admin/search/reindex-all",
    summary = "전체 포스트 재색인 트리거",
    description = "모든 포스트를 Meilisearch에 재색인합니다. (Admin 전용)",
    responses(
        (status = 200, description = "재색인 작업이 성공적으로 시작됨", body = AdminTaskResponse),
        (status = 401, description = "인증 실패"),
        (status = 403, description = "Admin 권한 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"
)]
pub async fn reindex_all_posts(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<AccessTokenClaims>,
) -> Result<AdminTaskResponse, StatusCode> {
    match service_reindex_all_posts(&app_state, token_data.sub).await {
        Ok(data) => Ok(AdminTaskResponse {
            success: true,
            message: "전체 포스트 재색인 작업이 시작되었습니다".to_string(),
            data: Some(data),
        }),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}