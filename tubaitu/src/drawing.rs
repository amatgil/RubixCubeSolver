use m_per_n::*;
use crate::*;

use std::fs::{self, File};
use std::io::Write;

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

const DRAWING_PIECE_RADIUS: f64 = 10.0;
impl Cube2 {
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

pub fn draw_sequence(file_prefix: &str, starting_cube: &Cube2, moves: Vec<Move>, n_in_between_frames: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut cube: Cube2 = starting_cube.clone();

    for (i, mov) in moves.iter().enumerate() {
        let i = i * n_in_between_frames;
        for inbetween_index in 0..n_in_between_frames {
            let lerp_t = inbetween_index as f64 / n_in_between_frames as f64;
            let filename = format!("{file_prefix}_{:>04}", i + inbetween_index);

            let svg: String = get_svg(&cube, &mov, lerp_t);
            
            let mut file: fs::File = fs::File::create(filename)?;
            file.write(svg.as_bytes())?;
        }
        cube.make_move(mov);
    }

    todo!()

}

fn get_svg(cube: &Cube2, mov: &Move, lerp_t: f64) -> String {
    let points = cube.to_points().pieces; // Un array de 8 DrawablePieces, que contenen els seus punts
    
    // Recorda que el radi és DRAWING_PIECE_RADIUS
    format!("{cube} with {mov:?} at with lerp value {lerp_t}")
}
