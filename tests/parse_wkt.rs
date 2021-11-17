use geos::{GResult, Geom, Geometry};

#[test]
fn parse_point() -> GResult<()> {
    let s = include_str!("data/Point.wkt");
    let point = Geometry::new_from_wkt(s)?;

    assert!(point.is_valid());
    assert!(point.is_simple()?);
    assert_eq!(point.get_type()?, "Point");

    Ok(())
}

#[test]
fn parse_polygon() -> GResult<()> {
    let s = include_str!("data/Polygon1.wkt");
    let poly = Geometry::new_from_wkt(s)?;

    assert!(poly.is_valid());
    assert!(poly.is_simple()?);
    assert_eq!(poly.get_type()?, "Polygon");

    Ok(())
}

#[test]
fn parse_geometry_collection() -> GResult<()> {
    let s = include_str!("data/GeometryCollection.wkt");
    let geo = Geometry::new_from_wkt(s)?;

    let geom2 = Geometry::new_from_wkt("POINT (35 38)").expect("Invalid point");

    assert!(geo.is_valid());
    assert!(geo.is_simple()?);
    assert!(geo.contains(&geom2)?);
    assert_eq!(geo.get_type()?, "GeometryCollection");

    Ok(())
}
