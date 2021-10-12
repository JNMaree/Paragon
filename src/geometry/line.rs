use crate::geometry::base::{*};
use crate::geometry::point::Point;

pub struct Line {
    pub p1: Point,
    pub p2: Point
}

pub struct LineLoop {
    pub n_lines: pCount,
    pub vec_lines: Vec<Line>
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }

    pub fn length() -> Length {

    }
}