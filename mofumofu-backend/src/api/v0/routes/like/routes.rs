use crate::api::v0::routes::like::check_like_status::check_like_status;
use crate::api::v0::routes::like::create_like::create_like;
use crate::api::v0::routes::like::delete_like::delete_like;
use crate::middleware::auth::access_jwt_auth;
use crate::state::AppState;
use axum::{middleware::from_fn, routing::{delete as axum_delete, post}, Router};

pub fn like_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/like",
            post(create_like).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/like",
            axum_delete(delete_like).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/like/status",
            post(check_like_status).route_layer(from_fn(access_jwt_auth)),
        )
}