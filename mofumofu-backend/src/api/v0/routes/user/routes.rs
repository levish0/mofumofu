use crate::api::v0::routes::user::create_user::create_user;
use crate::api::v0::routes::user::get_my_profile::get_my_profile;
use crate::api::v0::routes::user::get_profile::get_profile;
use crate::api::v0::routes::user::update_profile::update_profile;
use crate::api::v0::routes::user::upload_avatar::upload_avatar;
use crate::api::v0::routes::user::upload_banner::upload_banner;
use crate::middleware::auth::access_jwt_auth;
use crate::state::AppState;
use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post, put};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/user/profile", post(get_profile))
        .route("/user", post(create_user))
        // 보호된 사용자 프로필 API
        .route(
            "/user/my_profile",
            get(get_my_profile).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/user/profile",
            put(update_profile).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        // 이미지 업로드 API (8MB 제한)
        .route(
            "/user/profile/image",
            post(upload_avatar)
                .layer(DefaultBodyLimit::max(8 * 1024 * 1024)) // 8MB
                .route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/user/profile/banner",
            post(upload_banner)
                .layer(DefaultBodyLimit::max(8 * 1024 * 1024)) // 8MB
                .route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
}
