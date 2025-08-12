use super::trending_hashtags::trending_hashtags;
use crate::state::AppState;
use axum::routing::post;
use axum::Router;

pub fn hashtag_routes() -> Router<AppState> {
    Router::new().route("/trending", post(trending_hashtags))
}
