mod lib;
use lib::data::{Point, Segment};
use lib::influence_zone;
use std::time::SystemTime;

fn main() {
    let query_point: Point = Point { x: 3.0, y: 3.0 };
    let interest_points = vec![
        Point { x: 1.0, y: 1.0 },
        Point { x: 5.0, y: 5.0 },
        Point { x: 5.0, y: 1.0 },
    ];
    let bound = 10.0;
    let k = 2;
    let object: Point = Point { x: 6.0, y: 1.0 };

    let start = SystemTime::now();
    let zone: Vec<Segment> = influence_zone::compute(&query_point, &interest_points, k, bound);
    let intersect: bool = influence_zone::query(&query_point, &zone, &object);
    println!("{:?}", start.elapsed().unwrap());
}
