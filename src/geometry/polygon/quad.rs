use crate::geometry::point::Point;
use crate::geometry::line::{Line, LineLoop};
use crate::geometry::surface::Surface;
use crate::geometry::uid::pUID;

pub struct Quad {

    // Define the Vertices of the Quadrilateral Polygon
    pub p: [Point; 4],

    // Define the Lines/Sides of the Quadrilateral Polygon
    pub l: [Line; 4],

    // Define the Line Loop for the Quadrilateral Polygon
    pub ll: LineLoop,

    // Define the Bounded Surface for the Quadrilateral Polygon
    pub sur: Surface,

    // Define a Unique Identifier for this polygon
    uid: pUID
}