use crate::*;
use shared::PieceRotation;
use geo::{Polygon, LineString, Coord, BooleanOps, CoordsIter, Centroid};
use m_per_n::{Vec3, MatRow, Matrix};
use std::cmp::Ordering;
use std::f64::consts::PI;

#[derive(Copy, Clone)]
pub struct DrawablePiece {
    vertices: [Vertex; 8],
    faces: [Stiker; 6],
    drawing_order: [u8; 6],
}

impl DrawablePiece {
    pub fn new(center: Vec3, radius: f64, rotation:PieceRotation) -> Self{
        let mut vertices: [Vertex; 8] = [Vertex::default(); 8];
        
        let r = radius;

        for i in 0..vertices.len() {
            vertices[i]._3d = match i {
                0 => Vec3::new( r, -r,  r),
                1 => Vec3::new( r,  r,  r),
                2 => Vec3::new(-r,  r,  r),
                3 => Vec3::new(-r, -r,  r),
                4 => Vec3::new( r, -r, -r),
                5 => Vec3::new( r,  r, -r),
                6 => Vec3::new(-r,  r, -r),
                7 => Vec3::new(-r, -r, -r),
                _ => todo!(),
            }
        }

        let mut faces: [[Vertex; 4]; 6] = [[Vertex::default(); 4]; 6];

        for i in 0..faces.len() {
            let positions = get_vertices_in_face(i);
            for j in 0..positions.len() {
                faces[i][j] = vertices[positions[j]];
            }
        }

        let mut stikers: [Stiker; 6] = [Stiker::default(); 6];

        for i in 0..stikers.len() {
            stikers[i] = Stiker::new(faces[i], rotation.to_color_sequence()[i]);
        } 

        DrawablePiece{
            vertices:vertices,
            faces: stikers,
            drawing_order: [0; 6],
        }
    }


    pub fn apply_rotation(&mut self, mov: Move, mut lerp_t: f64) {
        if mov.side() == MoveSide::L || mov.side() == MoveSide::D || mov.side() == MoveSide::B {
            lerp_t *= -1.0;
        }
    
        lerp_t *= if mov.is_prime() { -1.0 } else { 1.0 };
    
        let cos = (lerp_t * PI / 2.0).cos();
        let sin = (lerp_t * PI / 2.0).sin();
        let matrix: Matrix<3, 3> = match mov.side() {
            MoveSide::R | MoveSide::L => Matrix::<3, 3>([
                MatRow::<3>([1.0, 0.0, 0.0]),
                MatRow::<3>([0.0, cos, sin]),
                MatRow::<3>([0.0, -sin, cos]),
            ]),
    
            MoveSide::U | MoveSide::D => Matrix::<3, 3>([
                MatRow::<3>([cos, sin, 0.0]),
                MatRow::<3>([-sin, cos, 0.0]),
                MatRow::<3>([0.0, 0.0, 1.0]),
            ]),
    
            MoveSide::F | MoveSide::B => Matrix::<3, 3>([
                MatRow::<3>([cos, 0.0, sin]),
                MatRow::<3>([0.0, 1.0, 0.0]),
                MatRow::<3>([-sin, 0.0, cos]),
            ]),
        };

        for i in 0..self.vertices.len() {
            let col_vec: Matrix<3, 1> = self.vertices[i]._3d.into(); // Write as column vector
            self.vertices[i]._3d = (matrix * col_vec).into();
        }

        self.update_stikers();

    }


    pub fn project_vertices(&self, camera: Camera, light_dir: Vec3) {
        todo!();
    }

    pub fn update_stikers(&mut self) {
        for i in 0..self.faces.len() {
            let positions = get_vertices_in_face(i);
            for j in 0..positions.len() {
                self.faces[i].vertices[j] = self.vertices[j];
            }
        }
    }

    pub fn find_displaying_order(&self, camera: Camera) {
        todo!();
    }

    pub fn get_outline_polygon(&self) -> Polygon {
        todo!();
    }

    pub fn get_overlap_centroid_2d(&self, piece: DrawablePiece, camera: Camera) -> Option<Coord> {
        todo!();
    }

    pub fn get_intersections_with_ray(&self, ray: Ray) -> Vec<Vec3> {
        todo!();
    }

    pub fn is_in_front(&self, piece: DrawablePiece, camera: Camera) -> Option<Ordering> {
        todo!();
    }

    pub fn get_drawing_data(&self) -> Vec<Polygon> {
        todo!();
    }
}

fn get_vertices_in_face(face: usize) -> [usize; 4] {
    match face {
        SIDE_RIGHT  => FACE_RIGHT_SEQ_CYCLE,
        SIDE_LEFT  => FACE_LEFT_SEQ_CYCLE,
        SIDE_TOP    => FACE_UP_SEQ_CYCLE,
        SIDE_DOWN   => FACE_DOWN_SEQ_CYCLE,
        SIDE_FRONT   => FACE_FRONT_SEQ_CYCLE,
        SIDE_BACK   => FACE_BACK_SEQ_CYCLE,
        _ => panic!(),
    }
}