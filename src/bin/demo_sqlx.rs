use geozero::wkb;
use post_gis::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = post_gis::get_connection_pool().await?;

    let row: (wkb::Decode<geo_types::Geometry<f64>>,) =
        sqlx::query_as("SELECT 'SRID=4326;POLYGON ((0 0, 2 0, 2 2, 0 2, 0 0))'::geometry")
            .fetch_one(&pool)
            .await?;
    let value = row.0;
    if let Some(geo_types::Geometry::Polygon(poly)) = value.geometry {
        assert_eq!(
            *poly.exterior(),
            vec![(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0), (0.0, 0.0)].into()
        );
    }

    // Insert geometry
    let geom: geo_types::Geometry<f64> = geo::Point::new(10.0, 20.0).into();
    let _ =
        sqlx::query("INSERT INTO point2d (datetimefield,geom) VALUES(now(),ST_SetSRID($1,4326))")
            .bind(wkb::Encode(geom))
            .execute(&pool)
            .await?;

    Ok(())
}
