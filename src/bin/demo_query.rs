use geozero::{wkb, ToWkt};
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn blob_query() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap())
        .await?;

    let row: (Vec<u8>,) =
        sqlx::query_as("SELECT 'SRID=4326;POLYGON ((0 0, 2 0, 2 2, 0 2, 0 0))'::geometry::bytea")
            .fetch_one(&pool)
            .await?;

    let wkt = wkb::Ewkb(row.0).to_wkt().expect("to_wkt failed");
    assert_eq!(&wkt, "POLYGON((0 0,2 0,2 2,0 2,0 0))");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    blob_query().await
}
