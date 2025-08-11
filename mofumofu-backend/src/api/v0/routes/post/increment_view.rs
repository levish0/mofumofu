use crate::dto::auth::internal::anonymous_user::AnonymousUserContext;
use crate::dto::post::request::GetPostByHandleAndSlugRequest;
use crate::service::error::errors::Errors;
use crate::service::post::increment_view_service::service_increment_view;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[utoipa::path(
    post,
    path = "/v0/post/view",
    request_body = GetPostByHandleAndSlugRequest,
    responses(
        (status = StatusCode::NO_CONTENT, description = "View count incremented successfully"),
        (status = StatusCode::NOT_FOUND, description = "Post not found"),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("anonymous_id_cookie" = [])
    ),
    tag = "Post"
)]
pub async fn increment_view(
    State(state): State<AppState>,
    Extension(anonymous_ctx): Extension<AnonymousUserContext>,
    ValidatedJson(req_body): ValidatedJson<GetPostByHandleAndSlugRequest>,
) -> Result<impl IntoResponse, Errors> {
    println!("{}", anonymous_ctx.anonymous_user_id);
    service_increment_view(
        &state,
        &state.conn,
        &req_body.handle,
        &req_body.slug,
        Some(&anonymous_ctx.anonymous_user_id),
    ).await?;

    Ok(StatusCode::NO_CONTENT)
}