use geos::{Geom, Geometry};

#[test]
fn parse_point() {
    let s = include_str!("data/Point.wkt");
    let point = Geometry::new_from_wkt(s).expect("Invalid geometry");

    assert!(point.is_valid());
    assert!(point.is_simple().unwrap());
    assert_eq!(point.get_type().unwrap(), "Point");
}

#[test]
fn parse_polygon() {
    let s = include_str!("data/Polygon1.wkt");
    let poly = Geometry::new_from_wkt(s).expect("Invalid geometry");

    assert!(poly.is_valid());
    assert!(poly.is_simple().unwrap());
    assert_eq!(poly.get_type().unwrap(), "Polygon");
}

#[test]
fn parse_geometry_collection() {
    let s = include_str!("data/GeometryCollection.wkt");
    let geo = Geometry::new_from_wkt(s).expect("Invalid geometry");

    assert!(geo.is_valid());
    assert!(geo.is_simple().unwrap());
    assert_eq!(geo.get_type().unwrap(), "GeometryCollection");
}
