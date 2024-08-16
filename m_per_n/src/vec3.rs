use std::ops::*;
use core::fmt;

use crate::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
impl Vec3 {
    pub const ZERO: Self     = Vec3 { x:  0.0, y:  0.0, z:  0.0 };
    pub const ONE: Self      = Vec3 { x:  1.0, y:  1.0, z:  1.0 };
    pub const NEG_ONE: Self  = Vec3 { x: -1.0, y: -1.0, z: -1.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Vec3{
        Vec3{ x, y, z }
    }
    pub fn abs(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    pub fn normalize(&self) -> Option<Vec3> {
        if self.abs() < FLOAT_EPSILON { None }
        else {
            Some(Vec3 {
                x: self.x / self.abs(),
                y: self.y / self.abs(),
                z: self.z / self.abs(),
            })
        }
    }
    pub fn cross_product(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.y*rhs.x
        }
    }

    pub fn dot_product(self, rhs: Vec3) -> f64 {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }

}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
         }
    }    
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self {
        Vec3 { 
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }    
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 { 
            x: -self.x,
            y: -self.y,
            z: -self.z,
            }
    }    
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x * rhs, 
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }    
}
