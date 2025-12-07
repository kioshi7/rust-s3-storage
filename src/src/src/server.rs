use axum::{routing::get, Router};
use std::path::PathBuf;

pub fn router(data_dir: PathBuf) -> Router {
    async fn health() -> &'static str {
        "OK"
    }

    Router::new()
        .route("/health", get(health))
        .with_state(data_dir)
}
