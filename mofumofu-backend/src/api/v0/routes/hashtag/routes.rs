use super::trending_hashtags::trending_hashtags;
use axum::routing::post;
use axum::Router;
use crate::state::AppState;

pub fn hashtag_routes() -> Router<AppState> {
    Router::new()
        .route("/trending", post(trending_hashtags))
}