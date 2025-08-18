use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::comment::request::create_comment::CreateCommentRequest;
use crate::service::comment::create_comment::service_create_comment;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/comment",
    request_body = CreateCommentRequest,
    responses(
        (status = StatusCode::NO_CONTENT, description = "Comment created successfully"),
        (status = StatusCode::NOT_FOUND, description = "Post not found or Parent comment not found"),
        (status = StatusCode::BAD_REQUEST, description = "Invalid parent comment or Cannot reply to deleted comment"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Comment"
)]
pub async fn create_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<CreateCommentRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received request to create comment: {:?}", payload);
    let user_uuid = claims.sub.clone();

    service_create_comment(&state.conn, &user_uuid, payload).await?;

    Ok(StatusCode::NO_CONTENT)
}