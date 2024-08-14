use crate::*;
use m_per_n::Vec3;

use shared::{Move};
use tubaitu::{Cube2};

const PIECE_RADIUS: f64 = 10.0;

pub struct DrawableCube2 {
    pieces: [DrawablePiece; 8],
    drawing_order: [u8; 8],
}

impl DrawableCube2 {
    pub fn new(cube: Cube2, mov: Move, lerp_t: f64) -> Self{

        let mut pieces: [DrawablePiece;8] = [DrawablePiece::new(Vec3::ZERO, 0.0 , PieceRotation::BO);8];
        let r = PIECE_RADIUS;

        for (i, piece_) in cube.pieces.iter().enumerate() {
            let center: Vec3 = match i {
                0 => Vec3::new( r, -r,  r),
                1 => Vec3::new( r,  r,  r),
                2 => Vec3::new(-r,  r,  r),
                3 => Vec3::new(-r, -r,  r),
                4 => Vec3::new( r, -r, -r),
                5 => Vec3::new( r,  r, -r),
                6 => Vec3::new(-r,  r, -r),
                7 => Vec3::new(-r, -r, -r),
                _ => panic!()
            };
            pieces[i] = DrawablePiece::new(center, r , piece_.rotation);
        };

        DrawableCube2{
            pieces: pieces,
            drawing_order: [0; 8],
        }
    }

    pub fn find_displaying_order(&self, camera: Camera) {
        todo!();
    }

    pub fn get_drawing_data(&self, camera: Camera, light_dir: Vec3) -> Vec<Quadrilateral> {
        todo!();
    }
}