use crate::dto::report::request::create_report::CreateReportRequest;
use crate::dto::report::response::create_report::CreateReportResponse;
use crate::service::error::errors::Errors;
use crate::service::report::create_report::service_create_report;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/report",
    request_body = CreateReportRequest,
    responses(
        (status = 201, description = "Report created successfully", body = CreateReportResponse),
        (status = StatusCode::BAD_REQUEST, description = "Invalid request"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "Report"
)]
pub async fn create_report(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateReportRequest>,
) -> Result<CreateReportResponse, Errors> {
    info!("Received request to create report: {:?}", payload);

    let response = service_create_report(&state.conn, payload).await?;

    Ok(response)
}
