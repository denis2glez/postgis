use geo::convex_hull::ConvexHull;
use geo::{line_string, polygon, LineString, Polygon};

#[test]
fn convex_hull() {
    // An L shape
    let poly = polygon![
        (x: 0.0, y: 0.0),
        (x: 4.0, y: 0.0),
        (x: 4.0, y: 1.0),
        (x: 1.0, y: 1.0),
        (x: 1.0, y: 4.0),
        (x: 0.0, y: 4.0),
        (x: 0.0, y: 0.0),
    ];

    // Calculate the polygon's convex hull
    let hull = poly.convex_hull();

    assert_eq!(
        hull.exterior(),
        &line_string![
            (x: 4.0, y: 0.0),
            (x: 4.0, y: 1.0),
            (x: 1.0, y: 4.0),
            (x: 0.0, y: 4.0),
            (x: 0.0, y: 0.0),
            (x: 4.0, y: 0.0),
        ]
    );
}

#[test]
fn polygon_exterior() {
    let polygon = Polygon::new(LineString::from(vec![(0., 0.), (1., 1.), (1., 0.)]), vec![]);

    assert_eq!(
        polygon.exterior(),
        &LineString::from(vec![(0., 0.), (1., 1.), (1., 0.), (0., 0.),])
    );
}
