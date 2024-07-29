use std::time::Instant;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;

use crate::repo::Repo;

pub fn router(repo: Repo) -> Router {
    let state = AppState {
        start_time: Instant::now(),
        repo,
    };
    Router::new()
        .route("/healthz", get(health_check))
        .route("/users", post(create_user))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

#[derive(Clone)]
struct AppState {
    start_time: Instant,
    repo: Repo,
}

#[derive(Serialize)]
struct HealthCheckResp {
    uptime_secs: u64,
}

// Performs a shallow health check.
async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    Json(HealthCheckResp {
        uptime_secs: state.start_time.elapsed().as_secs(),
    })
}

async fn create_user(State(state): State<AppState>) -> impl IntoResponse {
    // TODO: use a standard error format and don't unwrap
    state.repo.insert_user().await.unwrap();
}
