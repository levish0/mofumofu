use crate::dto::post::request::GetPostsAroundPageRequest;
use crate::dto::post::response::GetPostsResponse;
use crate::service::error::errors::Errors;
use crate::service::post::get_posts::service_get_posts_around_page;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/posts/around",
    request_body = GetPostsAroundPageRequest,
    responses(
        (status = StatusCode::OK, description = "Posts around target page retrieved successfully", body = GetPostsResponse),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "Post"
)]
pub async fn get_posts_around_page(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<GetPostsAroundPageRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!(
        "Received POST request to get posts around page: {:?}",
        payload
    );

    let response = service_get_posts_around_page(&state.conn, &state.meilisearch, payload).await?;

    Ok(response)
}
