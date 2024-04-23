
mod mats;
pub use mats::*;

mod vec3;
pub use vec3::*;

#[derive(Clone, Debug, Default, Copy)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64
}

impl Point { pub fn new(x: f64, y: f64, z: f64) -> Point { Point { x, y, z } } }

