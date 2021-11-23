use criterion::{black_box, criterion_group, criterion_main, Criterion};
use geo::{contains::Contains, polygon, Point};

fn bench_geo_contains(c: &mut Criterion) {
    c.bench_function("Point in polygon", |bencher| {
        let polygon = polygon![
            (x: 0.0, y: 0.0),
            (x: 1.0, y: 0.0),
            (x: 1.0, y: 1.0),
            (x: 0.0, y: 0.0),
        ];
        let in_candidate = Point::new(0.5, 0.1);
        bencher.iter(|| {
            black_box(black_box(&polygon).contains(black_box(&in_candidate)));
        });
    });

    c.bench_function("Point outside polygon", |bencher| {
        let polygon = polygon![
            (x: 0.0, y: 0.0),
            (x: 1.0, y: 0.0),
            (x: 1.0, y: 1.0),
            (x: 0.0, y: 0.0),
        ];
        let out_candidate = Point::new(2.0, 2.0);
        bencher.iter(|| {
            black_box(black_box(&polygon).contains(black_box(&out_candidate)));
        });
    });
}

criterion_group!(benches, bench_geo_contains);
criterion_main!(benches);
