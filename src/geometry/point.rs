use crate::geometry::base::{*};
use crate::geometry::uid::pUID;
use num_traits::pow;

pub struct Point {
    pub x: pCoordinate,
    pub y: pCoordinate,
    pub z: pCoordinate,
    uid: pUID
}

impl Point {
    // Constructor
    pub fn new(x: pCoordinate, y: pCoordinate, z: pCoordinate) -> Point {
        return Point(x, y, z);
    }

    // Calculate the displacement to the specified point
    pub fn displacement_to(&self, x: pCoordinate, y: pCoordinate, z: pCoordinate) -> pCoordinate {
        let x_diff = self.x - x;
        let y_diff = self.y - y;
        let z_diff = self.z - z;
        return sqrt(x_diff*x_diff + y_diff*y_diff + z_diff*z_diff)
    }
}

