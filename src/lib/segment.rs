use super::data::{Point, Segment};

pub fn generate(points: &Vec<Vec<Point>>) -> Vec<Segment> {
  let mut segments: Vec<Segment> = Vec::new();
  for point in points {
    segments.extend(compute(&point));
  }
  segments
}

pub fn compute(points: &Vec<Point>) -> Vec<Segment> {
  let mut index = 0;
  let mut segments: Vec<Segment> = Vec::new();

  while index < points.len() - 1 {
    segments.push(Segment {
      start: points[index].clone(),
      end: points[index + 1].clone(),
    });
    index = index + 1;
  }

  segments
}

pub fn mid_point(segment: &Segment) -> Point {
  let x: f64 = (segment.start.x + segment.end.x) / 2.0;
  let y: f64 = (segment.start.y + segment.end.y) / 2.0;
  Point { x, y }
}
