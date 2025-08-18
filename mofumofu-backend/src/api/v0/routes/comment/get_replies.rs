use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::comment::request::get_comments::GetRepliesRequest;
use crate::dto::comment::response::get_comments::GetRepliesResponse;
use crate::service::comment::get_replies::service_get_replies;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Extension;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/comment/replies",
    request_body = GetRepliesRequest,
    responses(
        (status = 200, description = "Replies retrieved successfully", body = GetRepliesResponse),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Comment"
)]
pub async fn get_replies(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<GetRepliesRequest>,
) -> Result<GetRepliesResponse, Errors> {
    info!("Received request to get replies: {:?}", payload);
    let user_uuid = Some(&claims.sub);

    let response = service_get_replies(&state.conn, user_uuid, payload).await?;

    Ok(response)
}