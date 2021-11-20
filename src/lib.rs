use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub type Result<T> = std::result::Result<T, sqlx::Error>;

/// Returns an asynchronous pool of SQLx Postgres connections.
pub async fn get_connection_pool() -> Result<PgPool> {
    // Load environment from `.env` file
    dotenv().ok();

    Ok(PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await?)
}
