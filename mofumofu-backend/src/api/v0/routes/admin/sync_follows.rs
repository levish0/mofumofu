use axum::{extract::State, http::StatusCode, Extension};


use crate::{
    dto::admin::response::AdminTaskResponse,
    dto::auth::internal::access_token::AccessTokenClaims,
    service::admin::sync_follows::service_sync_follows,
    state::AppState,
};


/// 유저 팔로우 수 동기화
#[utoipa::path(
    post,
    path = "/v0/admin/sync/follows",
    summary = "유저 팔로우 수 동기화",
    description = "모든 유저의 팔로워/팔로잉 수를 실제 데이터와 동기화합니다. (Admin 전용)",
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
pub async fn sync_follows(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<AccessTokenClaims>,
) -> Result<AdminTaskResponse, StatusCode> {
    match service_sync_follows(&app_state, token_data.sub).await {
        Ok(data) => Ok(AdminTaskResponse {
            success: true,
            message: "팔로우 수 동기화 작업이 시작되었습니다".to_string(),
            data: Some(data),
        }),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}