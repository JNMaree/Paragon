#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::geometry::base::{*};
use crate::geometry::axis::Axis;
use crate::geometry::point::Point;
use crate::geometry::line::Line;

pub struct Plane {
    pub origin:Point,
    pub bX_axis:Axis,
    pub bY_axis:Axis
}

impl Plane {

}