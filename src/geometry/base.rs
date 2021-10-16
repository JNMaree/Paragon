#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ops;
use std::process::Output;

use std::intrinsics::sqrtf32;

pub type pCoordinate = f32;
pub type pCount = i16;

pub struct pUnitVector {
    pub i: pCoordinate,
    pub j: pCoordinate,
    pub k: pCoordinate
}


pub fn sqrt(var: pCoordinate) -> pCoordinate {
    return (var as pCoordinate).sqrt()
}
