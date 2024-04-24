use std::ops::*;

#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    pub const ZERO: Self     = Vec3 { x:  0.0, y:  0.0, z:  0.0 };
    pub const ONE: Self      = Vec3 { x:  1.0, y:  1.0, z:  1.0 };
    pub const NEG_ONE: Self  = Vec3 { x: -1.0, y: -1.0, z: -1.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Vec3{
        Vec3{x:x,y:y,z:z}
    }
    pub fn abs(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    pub fn normalize(&self) -> Option<Vec3> {
        if self.abs() == 0.0 { None }
        else {
            Some(Vec3 {
                x: self.x / self.abs(),
                y: self.y / self.abs(),
                z: self.z / self.abs(),
            })
        }
    }
    pub fn cross_product(lhs: Vec3, rhs: Vec3) -> Vec3 {
        return Vec3 {
            x: lhs.y*rhs.z - lhs.z*rhs.y,
            y: lhs.z*rhs.x - lhs.x*rhs.z,
            z: lhs.x*rhs.y - lhs.y*rhs.x
        };
    }

    pub fn dot_product(lhs: Vec3, rhs: Vec3) -> f64 {
        lhs.x*rhs.x + lhs.y*rhs.y + lhs.z*rhs.z
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