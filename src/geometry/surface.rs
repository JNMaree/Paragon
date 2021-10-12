use crate::geometry::base::{*};
use crate::geometry::line::LineLoop;
use crate::geometry::plane::Plane;

pub struct Surface {
    pub line_loop: LineLoop,
    pub plane: Plane
}