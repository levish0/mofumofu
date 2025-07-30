use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;
use crate::dto::user::request::create::CreateUserRequest;
use crate::service::error::errors::Errors;
use crate::service::user::service_create_user;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;

// POST /user
#[utoipa::path(
    post,
    path = "/v0/user",
    request_body = CreateUserRequest,
    responses(
        (status = StatusCode::NO_CONTENT, description = "User created successfully"),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "User"
)]
pub async fn create_user(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received POST request to create user: {:?}", payload);

    service_create_user(&state.conn, payload).await?;

    Ok(StatusCode::NO_CONTENT)
}
