use crate::geometry::base::{*};
use crate::geometry::point::Point;

pub struct Line {
    pub p: [Point; 2],

    uid: pUID
}

pub struct LineLoop {
    pub n_lines: pCount,
    pub vec_lines: Vec<Line>
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { [p1; p2] }
    }

    pub fn length() -> Length {

    }
}