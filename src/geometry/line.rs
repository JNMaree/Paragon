use crate::geometry::point::Point;
use crate::geometry::{Coordinate, Length, sqrt};

pub struct Line {
    pub p1: Point,
    pub p2: Point
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }

    pub fn length() -> Length {

    }
}