

pub use shared::*;

mod mats;
pub use mats::*;

mod vec3;
pub use vec3::*;

#[derive(Clone, Debug, Default, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Debug, Clone, Copy)] // TODO: Check if Copy is hurting performance
pub struct Matrix<const NF: usize, const NC: usize> (
    pub [MatRow<NC>; NF]
);

#[derive(Debug, Clone, Copy)] // TODO: Check if Copy is hurting performance
pub struct MatRow<const NROWS: usize>(pub [f64; NROWS]);
/*
pub type Vec2 = Matrix<2, 1>;
pub type Vec3 = Matrix<3, 1>;
pub type Vec4 = Matrix<4, 1>;*/
