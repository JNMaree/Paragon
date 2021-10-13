use crate::geometry::base::pUID;
use crate::geometry::point::Point;
use crate::geometry::line::{Line, LineLoop};
use crate::geometry::surface::Surface;

pub struct Quad {

    // Define the Vertices of the Quadrilateral Polygon
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub p4: Point,

    // Define the Lines/Sides of the Quadrilateral Polygon
    pub l1: Line,
    pub l2: Line,
    pub l3: Line,
    pub l4: Line,

    // Define the Line Loop for the Quadrilateral Polygon
    pub ll: LineLoop,

    // Define the Bounded Surface for the Quadrilateral Polygon
    pub sur: Surface,

    // Define a Unique Identifier for this polygon
    uid: pUID
}