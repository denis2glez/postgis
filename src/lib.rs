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

#[cfg(test)]
mod tests {
    use geo::algorithm::{centroid::Centroid, coords_iter::CoordsIter};
    use geo::simplifyvw::SimplifyVWPreserve;
    use geo::{Geometry, Point};
    use geozero::geojson::GeoJson;
    use geozero::ToGeo;

    #[test]
    fn centroid() {
        let geojson = GeoJson(
            r#"{"type": "Polygon", "coordinates": [[[0, 0], [10, 0], [10, 6], [0, 6], [0, 0]]]}"#,
        );
        if let Ok(Geometry::Polygon(poly)) = geojson.to_geo() {
            assert_eq!(poly.centroid().unwrap(), Point::new(5.0, 3.0));
        }
    }

    #[test]
    fn simplify() {
        let geojson = GeoJson(
            r#"{"type": "LineString", "coordinates": [[1875038.447610231,-3269648.6879248763],[1874359.641504197,-3270196.812984864],[1874141.0428635243,-3270953.7840121365],[1874440.1778162003,-3271619.4315206874],[1876396.0598222911,-3274138.747656357],[1876442.0805243007,-3275052.60551469],[1874739.312657555,-3275457.333765534]]}"#,
        );
        if let Ok(Geometry::LineString(line)) = geojson.to_geo() {
            assert_eq!(line.coords_count(), 7);
            let simplified = line.simplifyvw_preserve(&800000.0);
            assert_eq!(simplified.coords_count(), 4);
        }
    }
}
