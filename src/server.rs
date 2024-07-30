use std::time::Instant;

use axum::{
    debug_handler,
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

#[derive(Debug, Serialize)]
struct User {}

#[debug_handler]
async fn create_user(State(state): State<AppState>) -> Result<Json<User>, crate::errors::Error> {
    state.repo.insert_user().await.map_err(|err| {
        // TODO: Detect a conflict
        crate::errors::Error {
            reason: crate::errors::Reason::Internal,
            message: format!("error creating user: {}", err),
        }
    })?;

    Ok(Json(User {}))
}
