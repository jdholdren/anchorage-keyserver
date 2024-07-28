use std::time::{Duration, Instant};

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::{info, info_span, Span};

pub fn router() -> Router {
    let state = AppState {
        start_time: Instant::now(),
    };
    Router::new()
        .route("/healthz", get(health_check))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

#[derive(Clone)]
struct AppState {
    start_time: Instant,
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
