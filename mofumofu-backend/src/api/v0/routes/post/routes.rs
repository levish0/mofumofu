use crate::api::v0::routes::post::create_post::create_post;
use crate::api::v0::routes::post::get_post_by_handle_and_slug::get_post_by_handle_and_slug;
use crate::api::v0::routes::post::get_posts::get_posts;
use crate::api::v0::routes::post::search_posts::search_posts;
use crate::api::v0::routes::post::upload_thumbnail::upload_thumbnail;
use crate::{middleware::auth::access_jwt_auth, state::AppState};
use axum::{Router, middleware::from_fn, routing::post};

pub fn post_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/post",
            post(create_post).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/post/thumbnail",
            post(upload_thumbnail).route_layer(from_fn(access_jwt_auth)),
        )
        .route("/post/get", post(get_post_by_handle_and_slug))
        .route("/posts", post(get_posts))
        .route("/posts/search", post(search_posts))
}
