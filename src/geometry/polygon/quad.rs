use crate::geometry::point::Point;
use crate::geometry::line::{Line, LineLoop};
use crate::geometry::surface::Surface;

pub struct Quad {

    // Define the Vertices of the Quadrilateral Polygon
    p1: Point,
    p2: Point,
    p3: Point,
    p4: Point,

    // Define the Lines/Sides of the Quadrilateral Polygon
    l1: Line,
    l2: Line,
    l3: Line,
    l4: Line,

    // Define the Line Loop for the Quadrilateral Polygon
    ll: LineLoop,

    // Define the Bounded Surface for the Quadrilateral Polygon
    sur: Surface

}