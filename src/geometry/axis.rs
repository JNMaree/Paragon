use crate::geometry::base::{*};
use crate::geometry::base::pUnitVector;
use crate::geometry::point::Point;

pub struct Axis {
    pub origin: Point,
    pub dir: pUnitVector
}