use m_per_n::Vec3;
use geo::Coord;

pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub camera_plane_distance: f64,
}

pub struct Quadrilateral {
    pub vertices: [Coord; 2],
    pub color: [u8; 3],
}

pub struct Ray {
    pub point: Vec3,
    pub direction: Vec3,
}

pub struct Plane {
    pub normal_vec: Vec3,
    pub point: Vec3,
}

impl Plane {
    fn find_intersection(&self, ray: Ray) -> Vec3 {
        todo!();
    }

    fn from_XYZ_to_XY(&self, point: Vec3) -> Coord {
        todo!();
    }
    
    fn from_XY_to_XYZ(&self, point: Coord) -> Vec3 {
        todo!();
    }
}