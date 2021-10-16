use crate::geometry::base::{*};
use crate::geometry::point::Point;
use crate::geometry::uid::pUID;

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
        pArr = [p1, p2];

        return Self
    }

    pub fn length() -> Length {

    }
}