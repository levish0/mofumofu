use crate::dto::post::request::GetPostByHandleAndSlugRequest;
use crate::dto::post::response::post_info::PostInfoResponse;
use crate::service::error::errors::Errors;
use crate::service::post::get_post_by_handle_and_slug::service_get_post_by_handle_and_slug;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;

#[utoipa::path(
    post,
    path = "/v0/post/get",
    request_body = GetPostByHandleAndSlugRequest,
    responses(
        (status = StatusCode::OK, description = "Post retrieved successfully", body = PostInfoResponse),
        (status = StatusCode::NOT_FOUND, description = "Post not found"),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "Post"
)]
pub async fn get_post_by_handle_and_slug(
    State(state): State<AppState>,
    ValidatedJson(request): ValidatedJson<GetPostByHandleAndSlugRequest>,
) -> Result<PostInfoResponse, Errors> {
    let post =
        service_get_post_by_handle_and_slug(&state.conn, &request.handle, &request.slug).await?;

    Ok(post)
}
