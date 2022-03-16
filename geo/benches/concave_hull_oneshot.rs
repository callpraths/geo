use geo::algorithm::concave_hull::ConcaveHull;
use geo::Polygon;
use iai::{black_box, main};

fn concave_hull_f32() -> Polygon<f32> {
    let line_string = geo_test_fixtures::norway_main::<f32>();
    black_box(&line_string).concave_hull(criterion::black_box(2.0))
}

fn concave_hull_f64() -> Polygon<f64> {
    let line_string = geo_test_fixtures::norway_main::<f64>();
    black_box(&line_string).concave_hull(criterion::black_box(2.0))
}

main!(concave_hull_f32, concave_hull_f64);
