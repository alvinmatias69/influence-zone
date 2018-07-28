use super::data::{Line, Point, Segment};
use super::perpendicular_bisector;
use super::intersection;
use super::segment;

pub fn compute(
  query_point: &Point,
  interest_points: &Vec<Point>,
  k: usize,
  bound: f64,
) -> Vec<Segment> {
  let bisector: Vec<Line> = perpendicular_bisector::generate(&query_point, &interest_points);
  let intersect: Vec<Vec<Point>> = intersection::generate(&bisector, bound);
  let segments: Vec<Segment> = segment::generate(&intersect);
  let mut query_segment: Segment;
  let mut count: usize;
  let mut zone: Vec<Segment> = Vec::new();

  for cur_segment in segments.into_iter() {
    query_segment = Segment {
      start: query_point.clone(),
      end: segment::mid_point(&cur_segment),
    };
    count = count_intersection(&query_segment, &bisector);
    if count == k {
      zone.push(cur_segment.clone());
    }
  }

  zone
}

fn count_intersection(segment: &Segment, bisector: &Vec<Line>) -> usize {
  let mut count: usize = 0;
  for line in bisector.into_iter() {
    if intersection::line_segment(&line, &segment) {
      count = count + 1;
    }
  }
  count
}

pub fn query(query_point: &Point, zone: &Vec<Segment>, object: &Point) -> bool {
  let mut intersect: bool = false;

  let query_segment: Segment = Segment {
    start: query_point.clone(),
    end: object.clone(),
  };

  for test_segment in zone.into_iter() {
    if !intersect && intersection::segment(&query_segment, &test_segment) {
      intersect = true;
    }
  }

  !intersect
}
