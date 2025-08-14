use crate::dto::user::request::create::CreateUserRequest;
use crate::service::error::errors::Errors;
use crate::service::user::service_create_user;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

// POST /user
#[utoipa::path(
    post,
    path = "/v0/user",
    request_body = CreateUserRequest,
    responses(
        (status = 204, description = "User created successfully"),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "Handle already exists"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error")
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
