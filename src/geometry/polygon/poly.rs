use crate::geometry::line::{Line, LineLoop};
use crate::geometry::point::Point;
use crate::geometry::surface::Surface;
use crate::geometry::uid::pUID;

pub struct Poly {

    pub p: Vec<Point>,

    pub l: Vec<Line>,

    pub ll: LineLoop,

    pub sur: Surface,

    uid: pUID
}

pub trait P {

}