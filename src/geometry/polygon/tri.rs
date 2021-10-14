use crate::geometry::base::pUID;
use crate::geometry::line::{Line, LineLoop};
use crate::geometry::point::Point;
use crate::geometry::surface::Surface;

pub struct Tri {

    pub p: [Point; 3],

    pub l: [Line; 3],

    pub ll: LineLoop,

    pub sur: Surface,

    uid: pUID
}