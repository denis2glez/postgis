use geo::{algorithm::centroid::Centroid, line_string};

fn main() {
    // Creates a LineString containing the given coordinates.
    let ls = geo::line_string![
        (x: 40.02f64, y: 116.34),
        (x: 41.02f64, y: 116.34),
    ];
    println!("Centroid {:?}", ls.centroid());
}
