use m_per_n::{Vec3, MatRow, Matrix};
use geo::Coord;

pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub camera_plane_distance: f64,
}

impl Camera {
    pub fn intersection_with_plane(&self, point: Vec3) -> Vec3 {
        let mut v: Vec3 = point - self.position;
        let mut n: Vec3 = self.direction;

        v = v.normalize().unwrap();
        n = n.normalize().unwrap();

        let l = self.camera_plane_distance/(v.dot_product(n));
        return self.position + v*l;
    }

    pub fn get_from_xyz_to_xy_matrix(&self) -> Matrix<3,3> {
        self.get_from_xy_to_xyz_matrix().inverse().unwrap()
    }
    
    pub fn get_from_xy_to_xyz_matrix(&self) -> Matrix<3,3> {
        let n = self.direction;
        let basis1 = Vec3::new(
            n.y * 0.0 - n.z * n.y,
            n.z * n.x - n.x * 0.0,
            n.x * n.y - n.y * n.x,
        );
        let n_i = basis1.normalize().unwrap();

        let basis2 = Vec3::new(
            n.y * basis1.z - n.z * basis1.y,
            n.z * basis1.x - n.x * basis1.z,
            n.x * basis1.y - n.y * basis1.x,
        );
        let n_j = basis2.normalize().unwrap();

        let transformation = Matrix::<3, 3>([
            MatRow::<3>([n_i.x, n_j.x, n.x]),
            MatRow::<3>([n_i.y, n_j.y, n.y]),
            MatRow::<3>([n_i.z, n_j.z, n.z]),
        ]);
        return transformation;
    }
}

#[derive(Default)]
pub struct Quadrilateral {
    pub vertices: [Coord; 4],
    pub color: [usize; 3],
}

pub struct Ray {
    pub point: Vec3,
    pub direction: Vec3,
}
