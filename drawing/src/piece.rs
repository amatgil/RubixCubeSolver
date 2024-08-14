use crate::*;
use shared::PieceRotation;
use geo::{Polygon, LineString, Coord, BooleanOps, CoordsIter, Centroid};
use m_per_n::Vec3;
use std::cmp::Ordering;

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

        faces[SIDE_RIGHT] = [
            vertices[P_TOP_RIGHT_FRONT], vertices[P_TOP_RIGHT_BACK], vertices[P_BOTTOM_RIGHT_BACK], vertices[P_BOTTOM_RIGHT_FRONT]
        ];
        faces[SIDE_FRONT] = [
            vertices[P_TOP_RIGHT_FRONT], vertices[P_BOTTOM_RIGHT_FRONT], vertices[P_BOTTOM_LEFT_FRONT], vertices[P_TOP_LEFT_FRONT]
        ];
        faces[SIDE_TOP] =  [
            vertices[P_TOP_RIGHT_FRONT], vertices[P_TOP_LEFT_FRONT], vertices[P_TOP_LEFT_BACK], vertices[P_TOP_RIGHT_BACK]
        ];
        faces[SIDE_LEFT] = [
            vertices[P_TOP_LEFT_FRONT], vertices[P_BOTTOM_LEFT_FRONT], vertices[P_BOTTOM_LEFT_BACK], vertices[P_TOP_LEFT_BACK]
        ];
        faces[SIDE_BACK] = [
            vertices[P_TOP_RIGHT_BACK], vertices[P_TOP_LEFT_BACK], vertices[P_BOTTOM_LEFT_BACK], vertices[P_BOTTOM_RIGHT_BACK], 
            ];
        faces[SIDE_DOWN] = [
            vertices[P_BOTTOM_RIGHT_FRONT], vertices[P_BOTTOM_RIGHT_BACK], vertices[P_BOTTOM_LEFT_BACK], vertices[P_BOTTOM_LEFT_FRONT]
        ];

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

    pub fn apply_rotation(&self, axis: u8, clockwhise: bool) {
        todo!();
    }

    pub fn project_vertices(&self, camera: Camera, light_dir: Vec3) {
        todo!();
    }

    pub fn update_stikers(&self) {
        todo!();
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