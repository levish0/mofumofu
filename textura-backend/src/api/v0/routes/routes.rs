use super::openapi::ApiDoc;
use crate::api::v0::routes::auth::auth::auth_routes;
use crate::service::error::errors::handler_404;
use crate::state::AppState;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// API + Swagger UI 라우터 통합
pub fn api_routes() -> Router<AppState> {
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/swagger.json", ApiDoc::openapi()))
        .nest("/v0", auth_routes())
        .fallback(handler_404)
}
