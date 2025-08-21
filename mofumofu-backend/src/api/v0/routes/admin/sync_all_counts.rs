use axum::{extract::State, http::StatusCode, Extension};


use crate::{
    dto::admin::response::AdminTaskResponse,
    dto::auth::internal::access_token::AccessTokenClaims,
    service::admin::sync_all_counts::service_sync_all_counts,
    state::AppState,
};


/// 전체 카운트 동기화
#[utoipa::path(
    post,
    path = "/v0/admin/sync/all",
    summary = "전체 카운트 동기화",
    description = "모든 카운트(좋아요, 팔로우)를 실제 데이터와 동기화합니다. (Admin 전용)",
    responses(
        (status = 200, description = "동기화 작업 시작됨", body = AdminTaskResponse),
        (status = 401, description = "인증 실패"),
        (status = 403, description = "Admin 권한 필요"),
        (status = 500, description = "서버 오류")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"
)]
pub async fn sync_all_counts(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<AccessTokenClaims>,
) -> Result<AdminTaskResponse, StatusCode> {
    match service_sync_all_counts(&app_state, token_data.sub).await {
        Ok(data) => Ok(AdminTaskResponse {
            success: true,
            message: "전체 카운트 동기화 작업이 시작되었습니다".to_string(),
            data: Some(data),
        }),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}