use crate::api::v0::routes::user::create_user::create_user;
use crate::api::v0::routes::user::get_my_profile::get_my_profile;
use crate::api::v0::routes::user::get_profile::get_profile;
use crate::middleware::auth::access_jwt_auth;
use crate::state::AppState;
use axum::Router;
use axum::routing::{get, post};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/user/profile", post(get_profile))
        .route("/user", post(create_user))
        // 보호된 사용자 프로필 API
        .route(
            "/user/my_profile",
            get(get_my_profile).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
}
