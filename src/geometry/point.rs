use crate::geometry::Coordinate;

pub struct Point {
    pub x:Coordinate,
    pub y:Coordinate,
    pub z:Coordinate
}

impl Point {
    pub fn new(x:Coordinate, y:Coordinate, z:Coordinate) -> Point {
        Point {x, y, z}
    }
}

