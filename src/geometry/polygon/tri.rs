use crate::geometry::line::{Line, LineLoop};
use crate::geometry::point::Point;
use crate::geometry::surface::Surface;
use crate::geometry::uid::pUID;

pub struct Tri {

    pub p: [Point; 3],

    pub l: [Line; 3],

    pub ll: LineLoop,

    pub sur: Surface,

    uid: pUID
}