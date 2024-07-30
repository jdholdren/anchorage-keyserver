#![allow(dead_code, unused_imports, unused_variables)]

mod errors;
mod repo;
mod server;

use std::env;

use repo::Repo;

use anyhow::{anyhow, bail, Context, Result};
use envconfig::Envconfig;
use sqlx::SqlitePool;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Envconfig)]
struct Config {
    // The location of the sqlite database.
    #[envconfig(from = "DB_FILE")]
    pub db_file: String,

    #[envconfig(from = "PORT")]
    pub http_port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = match Config::init_from_env() {
        Err(err) => {
            bail!("error parsing config from env: {}", err);
        }
        Ok(val) => val,
    };

    // Connect to the db
    let pool = SqlitePool::connect(&cfg.db_file).await?;

    run_server(cfg, pool).await
}

async fn run_server(cfg: Config, pool: SqlitePool) -> Result<()> {
    let repo = Repo::new(pool);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", cfg.http_port))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, server::router(repo))
        .await
        .map_err(|err| anyhow!("server error: {}", err))
}

async fn run_migrations(pool: SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("error running migrations")
}
