#[derive(Clone, Debug)]
pub struct Point {
  pub x: f64,
  pub y: f64,
}

#[derive(Clone, Debug)]
pub struct Segment {
  pub start: Point,
  pub end: Point,
}

#[derive(Clone, Debug)]
pub struct Line {
  pub m: f64,
  pub c: f64,
}
