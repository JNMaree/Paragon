use crate::geometry::line::{Line, LineLoop};
use crate::geometry::point::Point;
use crate::geometry::surface::Surface;

pub struct Tri {

    p1: Point,
    p2: Point,
    p3: Point,

    l1: Line,
    l2: Line,
    l3: Line,

    ll: LineLoop,

    sur: Surface

}