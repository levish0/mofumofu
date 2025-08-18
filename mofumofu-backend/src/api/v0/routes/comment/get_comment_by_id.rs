use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::comment::request::get_comment_by_id::GetCommentByIdRequest;
use crate::dto::comment::response::comment_info::CommentInfo;
use crate::service::comment::get_comment_by_id::service_get_comment_by_id;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Extension;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/comment/get",
    request_body = GetCommentByIdRequest,
    responses(
        (status = 200, description = "Comment retrieved successfully", body = CommentInfo),
        (status = 404, description = "Comment not found"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Comment"
)]
pub async fn get_comment_by_id(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<GetCommentByIdRequest>,
) -> Result<CommentInfo, Errors> {
    info!("Received request to get comment by id: {:?}", payload);
    let user_uuid = Some(&claims.sub);

    let response = service_get_comment_by_id(&state.conn, user_uuid, payload.comment_id).await?;

    Ok(response)
}