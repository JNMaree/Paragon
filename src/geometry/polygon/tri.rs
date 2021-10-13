use crate::geometry::base::pUID;
use crate::geometry::line::{Line, LineLoop};
use crate::geometry::point::Point;
use crate::geometry::surface::Surface;

pub struct Tri {

    pub p1: Point,
    pub p2: Point,
    pub p3: Point,

    pub l1: Line,
    pub l2: Line,
    pub l3: Line,

    pub ll: LineLoop,

    pub sur: Surface,

    uid: pUID
}