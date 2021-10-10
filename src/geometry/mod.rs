use std::intrinsics::sqrtf32;
use crate::geometry::base::BaseType_Dimension;

pub mod base;
pub mod point;
pub mod line;
pub mod quad;

type Base = BaseType_Dimension;
type Coordinate = BaseType_Dimension;
type Length = BaseType_Dimension;

pub fn sqrt(x: Base) -> Length {
    return (x as Base).sqrt()
}


