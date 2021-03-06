use geozero::{wkb, ToWkt};
use post_gis::Result;

pub async fn blob_query() -> Result<()> {
    let pool = post_gis::get_connection_pool().await?;

    let row: (Vec<u8>,) =
        sqlx::query_as("SELECT 'SRID=4326;POLYGON ((0 0, 2 0, 2 2, 0 2, 0 0))'::geometry::bytea")
            .fetch_one(&pool)
            .await?;

    let wkt = wkb::Ewkb(row.0).to_wkt().expect("to_wkt failed");
    assert_eq!(&wkt, "POLYGON((0 0,2 0,2 2,0 2,0 0))");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    blob_query().await
}
