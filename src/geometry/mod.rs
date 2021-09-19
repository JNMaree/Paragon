use std::intrinsics::sqrtf32;

pub mod point;
pub mod line;
pub mod quad;

type Base = f32;

type Coordinate = Base;
type Length = Base;

pub fn sqrt(x: Base) -> Length {
    return (x as Base).sqrt()
}
