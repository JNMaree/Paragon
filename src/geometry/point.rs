use crate::geometry::base::{*};

pub struct Point {
    pub x: pCoordinate,
    pub y: pCoordinate,
    pub z: pCoordinate,

    uid: pUID
}

impl Point {
    // Constructor
    pub fn new(x: pCoordinate, y: pCoordinate, z: pCoordinate) -> Point {
        Point {x, y, z}
    }

    // Calculate the sum of the squares of x, y, z
    pub fn sq_sum() -> pCoordinate {
        return
    }

    // Calculate the displacement from the specified point
    pub fn displacement(x: pCoordinate, y: pCoordinate, z: pCoordinate) -> pCoordinate {
        return
    }
}

