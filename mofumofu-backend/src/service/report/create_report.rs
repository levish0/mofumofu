use crate::dto::report::request::create_report::CreateReportRequest;
use crate::dto::report::response::create_report::CreateReportResponse;
use crate::repository::report::create_report::repository_create_report;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::ConnectionTrait;
use uuid::Uuid;

pub async fn service_create_report<C>(
    conn: &C,
    request: CreateReportRequest,
) -> ServiceResult<CreateReportResponse>
where
    C: ConnectionTrait,
{
    let created_report = repository_create_report(
        conn,
        None, // 익명 신고
        request.target_type,
        request.target_id,
        request.reasons,
        request.description,
    )
    .await?;

    Ok(CreateReportResponse {
        report_id: created_report.id,
    })
}
