use std::time::SystemTime;

use anyhow::Context;
use sqlx::SqlitePool;

/// Repo holds the repository for interacting with user records.
#[derive(Clone)]
pub struct Repo {
    pool: SqlitePool,
}

impl Repo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

// Errors that can occur while using the repository.
#[derive(Debug)]
pub enum Error {
    Internal(anyhow::Error),
    NotFound,
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self::Internal(value)
    }
}

// Represents a row in the `users` table.
struct UserRow {
    email: String,
    key_contents: String,
    confirmation_code: String,
    created_at: SystemTime,
    updated_at: SystemTime,
}

impl Repo {
    pub async fn insert_user(&self) -> Result<(), Error> {
        let conn = self
            .pool
            .acquire()
            .await
            .context("error aquiring connection")?;

        sqlx::query(
            "
        INSERT INTO users (email) VALUES ('james');
        ",
        )
        .execute(&self.pool)
        .await
        .context("error inserting user")?;

        Ok(())
    }
}
