use crate::api::v0::routes::auth::github::github_sign_in;
use crate::api::v0::routes::auth::google::google_sign_in;
use crate::api::v0::routes::auth::refresh::refresh;
use crate::api::v0::routes::auth::sign_in::sign_in;
use crate::api::v0::routes::auth::sign_out::sign_out;
use crate::middleware::auth::refresh_jwt_auth;
use crate::state::AppState;
use axum::routing::post;
use axum::Router;
use crate::api::v0::routes::auth::clear_refresh::clear_refresh;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/sign_in", post(sign_in))
        .route(
            "/auth/sign_out",
            post(sign_out).route_layer(axum::middleware::from_fn(refresh_jwt_auth)),
        )
        .route(
            "/auth/clear_refresh", post(clear_refresh)
        )
        .route(
            "/auth/refresh",
            post(refresh).route_layer(axum::middleware::from_fn(refresh_jwt_auth)),
        )
        .route("/auth/google", post(google_sign_in))
        .route("/auth/github", post(github_sign_in))
}
