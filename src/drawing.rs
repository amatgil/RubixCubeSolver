use crate::*;

use std::fs;
use std::io::Write;

#[derive(Clone, Debug, Default, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64
}

#[derive(Clone, Debug, Default, Copy)]
struct DrawablePiece {
    rotation: PieceRotation,
    center: Point,
    radius: f64,
}

#[derive(Clone, Debug, Default, Copy)]
struct DrawableCube {
    pieces: [DrawablePiece; 8],
}

impl Point { fn new(x: f64, y: f64, z: f64) -> Point { Point { x, y, z } } }

const DRAWING_PIECE_RADIUS: f64 = 10.0;
impl Cube {
    fn to_points(&self) -> DrawableCube {
        let r = DRAWING_PIECE_RADIUS;
        let mut drawable_pieces = [DrawablePiece::default(); 8 ];

        for (piece_idx, original_piece) in self.pieces.iter().enumerate() {
            let rotation: PieceRotation = original_piece.rotation;
            let center: Point = match piece_idx {
                P_TOP_RIGHT_FRONT    => Point::new( r, -r,  r),  
                P_TOP_RIGHT_BACK     => Point::new( r,  r,  r),
                P_TOP_LEFT_BACK      => Point::new(-r,  r,  r),
                P_TOP_LEFT_FRONT     => Point::new(-r, -r,  r),
                P_BOTTOM_RIGHT_FRONT => Point::new( r, -r, -r),  
                P_BOTTOM_RIGHT_BACK  => Point::new( r,  r, -r),
                P_BOTTOM_LEFT_BACK   => Point::new(-r,  r, -r),
                P_BOTTOM_LEFT_FRONT  => Point::new(-r, -r, -r),
                _ => unreachable!("Piece index no vàlid?"),
            };

            drawable_pieces[piece_idx] = DrawablePiece {
                center,
                radius: DRAWING_PIECE_RADIUS,
                rotation
            };
        }

    DrawableCube { pieces: drawable_pieces }
    }
}

pub fn draw_sequence(file_prefix: &str, cube: &Cube, moves: Vec<Move>, n_in_between_frames: usize) -> Result<(), Box<dyn std::error::Error>> {
    for (i, mov) in moves.iter().enumerate() {
        for inbetween_index in 0..n_in_between_frames {
            let filename = format!("{file_prefix}_{:>04}", i + inbetween_index);

            let svg: String = "Testing string".to_string();
            
            let mut file: fs::File = fs::File::create(filename)?;
            file.write(svg.as_bytes())?;
        }
    }

    todo!()

}
