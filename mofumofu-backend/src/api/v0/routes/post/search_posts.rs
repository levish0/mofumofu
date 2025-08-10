use crate::dto::post::request::SearchPostsRequest;
use crate::dto::post::response::GetPostsResponse;
use crate::service::error::errors::Errors;
use crate::service::post::search_posts::service_search_posts;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/posts/search",
    request_body = SearchPostsRequest,
    responses(
        (status = StatusCode::OK, description = "Posts search completed successfully", body = GetPostsResponse),
        (status = StatusCode::BAD_REQUEST, description = "Invalid search parameters"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Search service error")
    ),
    tag = "Post"
)]
pub async fn search_posts(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<SearchPostsRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received POST request to search posts: {:?}", payload);

    let response = service_search_posts(&state.conn, &state.meilisearch, payload).await?;

    Ok(response)
}