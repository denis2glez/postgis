use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:password@localhost/testdb")
        .await?;

    Ok(())
}
