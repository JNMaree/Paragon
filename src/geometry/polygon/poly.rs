use crate::geometry::line::{Line, LineLoop};
use crate::geometry::point::Point;
use crate::geometry::surface::Surface;

pub struct Poly {

    p: Vec<Point>,

    l: Vec<Line>,

    ll: LineLoop,

    sur: Surface
}