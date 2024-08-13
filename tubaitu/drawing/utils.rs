use m_per_n::Vec3;
use geo::Coord;

struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub camera_plane_distance: f64,
}

struct Quadrilateral {
    pub vertices: [Coord; 2],
    pub color: [u8; 3],
}

struct Ray {
    pub point: Vec3,
    pub direction: Vec3,
}

struct Plane {
    pub normal_vec: Vec3,
    pub point: Vec3,
}

impl Plane {
    fn find_intersection(ray: Ray) -> Vec3 {
        todo!();
    }

    fn from_XYZ_to_XY(point: Vec3) -> Coord {
        todo!();
    }
    
    fn from_XY_to_XYZ(point: Coord) -> Vec3 {
        todo!();
    }
}