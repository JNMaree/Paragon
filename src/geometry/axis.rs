use crate::geometry::base::{*};
use crate::geometry::base::pUnitVector;
use crate::geometry::point::Point;
use crate::geometry::uid::pUID;

pub struct Axis {
    pub origin: Point,
    pub dir: pUnitVector,

    uid: pUID
}