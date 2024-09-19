use std::fmt::Display;

pub use shared::*;

mod moving;
use moving::*;

/// A 3x3x3 Rubix's cube with the following orientation:
/// | Face  | Center Color |
/// |-------|--------------|
/// | Up    | Yellow       |
/// | Down  | White        |
/// | Left  | Green        |
/// | Right | Blue         |
/// | Front | Orange       |
/// | Back  | Red          |
///
/// Internally, it is represented by the 6 edges and 14 corners only. Their indicies are defined by the corresponding `ThreeByCorner` and `ThreeByEdge` enums.
/// Note that equality is direct here, unlike a 2x2: there are no symmetries to speak of because the centers ground us
/// The Default cube is the solved `YO` cube
#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct Cube3 {
    pieces: [Piece; 3*3*3 - 1 - 6], // 20: edges + corners
}


impl Drawable<27> for Cube3 {
    type DrawablePiece = ();

    fn to_points(self) -> [Self::DrawablePiece; 27] {
        todo!()
    }

    fn get_polys(&self, part_mov: Option<PartialMove>, width: usize, height: usize, scale: f64) -> Vec<Polygon> {
        todo!()
    }
}
impl Solvable for Cube3 {
    fn moves_of_adjacency() -> Vec<Move> {
        Vec::from([
            Move::R, Move::RP,
            Move::L, Move::LP,
            Move::F, Move::FP,
            Move::B, Move::BP,
            Move::U, Move::UP,
            Move::D, Move::DP
        ])
    }
    fn make_move(&mut self, m: Move) {
        make_three_by_three_move(self, m);
    }
}



pub const CUBE_PRINT_WIDTH: usize = 3*4 + 5 + 1;
pub const CUBE_PRINT_HEIGHT: usize = 3*3 + 3 + 1;
fn xy_to_idx(x: usize, y: usize) -> usize { y*CUBE_PRINT_WIDTH + x }

const RIGHT_PRINTING_CYCLE: [usize; 8] = [
    ThreeByCorner::YBO as usize,
    ThreeByEdge::YB    as usize,
    ThreeByCorner::YBR as usize,
    ThreeByEdge::OB    as usize,
    ThreeByEdge::BR    as usize,
    ThreeByCorner::WBO as usize,
    ThreeByEdge::WB    as usize,
    ThreeByCorner::WRB as usize,
];

const LEFT_PRINTING_CYCLE: [usize; 8] = [
    ThreeByCorner::YRG as usize,
    ThreeByEdge::YG    as usize,
    ThreeByCorner::YOG as usize,
    ThreeByEdge::RG    as usize,
    ThreeByEdge::GO as usize,
    ThreeByCorner::WGR    as usize,
    ThreeByEdge::WG as usize,
    ThreeByCorner::WOG    as usize,
];

const FRONT_PRINTING_CYCLE: [usize; 8] = [
    ThreeByCorner::YOG as usize,
    ThreeByEdge::YO    as usize,
    ThreeByCorner::YBO as usize,
    ThreeByEdge::GO    as usize,
    ThreeByEdge::OB    as usize,
    ThreeByCorner::WOG as usize,
    ThreeByEdge::WO    as usize,
    ThreeByCorner::WBO as usize,
];

const BACK_PRINTING_CYCLE: [usize; 8] = [
    ThreeByCorner::YBR as usize,
    ThreeByEdge::YR    as usize,
    ThreeByCorner::YRG as usize,
    ThreeByEdge::BR    as usize,
    ThreeByEdge::RG    as usize,
    ThreeByCorner::WRB as usize,
    ThreeByEdge::WR    as usize,
    ThreeByCorner::WGR as usize,
];

const UP_PRINTING_CYCLE: [usize; 8] = [
    ThreeByCorner::YRG as usize,
    ThreeByEdge::YR    as usize,
    ThreeByCorner::YBR as usize,
    ThreeByEdge::YG    as usize,
    ThreeByEdge::YB    as usize,
    ThreeByCorner::YOG as usize,
    ThreeByEdge::YO    as usize,
    ThreeByCorner::YBO as usize,
];

const DOWN_PRINTING_CYCLE: [usize; 8] = [
    ThreeByCorner::WOG as usize,
    ThreeByEdge::WO    as usize,
    ThreeByCorner::WBO as usize,
    ThreeByEdge::WG    as usize,
    ThreeByEdge::WB    as usize,
    ThreeByCorner::WGR as usize,
    ThreeByEdge::WR    as usize,
    ThreeByCorner::WRB as usize,
];

impl Display for Cube3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	let mut buffer: [u8; CUBE_PRINT_WIDTH*CUBE_PRINT_HEIGHT] =
	    [b' '; CUBE_PRINT_WIDTH*CUBE_PRINT_HEIGHT];


	for y in 0..CUBE_PRINT_HEIGHT - 1 { buffer[xy_to_idx(CUBE_PRINT_WIDTH - 1, y)] = b'\n' }
	for y in [4, 8] {
	    for x in 0..CUBE_PRINT_WIDTH - 1 { buffer[xy_to_idx(x, y)] = b'-' } 
	}

	for y in [0, CUBE_PRINT_HEIGHT - 1] {
	    for x in 5..8  { buffer[xy_to_idx(x, y)] = b'-' } 
	}

        print_add_face(&mut buffer, &self.pieces, 2, UP_PRINTING_CYCLE,    5, 1, Color::Yellow);
        print_add_face(&mut buffer, &self.pieces, 3, LEFT_PRINTING_CYCLE,  1, 5, Color::Green);
        print_add_face(&mut buffer, &self.pieces, 1, FRONT_PRINTING_CYCLE, 5, 5, Color::Orange);
        print_add_face(&mut buffer, &self.pieces, 0, RIGHT_PRINTING_CYCLE, 9, 5, Color::Blue);
        print_add_face(&mut buffer, &self.pieces, 5, DOWN_PRINTING_CYCLE,  5, 9, Color::White);
        print_add_face(&mut buffer, &self.pieces, 4, BACK_PRINTING_CYCLE, 13, 5, Color::Red);

	let s = std::str::from_utf8(&buffer).expect("invalid utf-8 sequence (should be impossible)");
        write!(f, "{s}")
    }
}
#[rustfmt::skip]
#[allow(clippy::identity_op)]
fn print_add_face(
    buffer: &mut [u8; CUBE_PRINT_WIDTH*CUBE_PRINT_HEIGHT],
    p: &[Piece; 20],
    n: usize,
    seq: [usize; 8],
    start_x: usize,
    start_y: usize,
    center_color: Color
) {
    buffer[xy_to_idx(start_x + 0, start_y + 0)] = p[seq[0]].to_color_sequence()[n].to_string().bytes().next().unwrap();
    buffer[xy_to_idx(start_x + 1, start_y + 0)] = p[seq[1]].to_color_sequence()[n].to_string().bytes().next().unwrap();
    buffer[xy_to_idx(start_x + 2, start_y + 0)] = p[seq[2]].to_color_sequence()[n].to_string().bytes().next().unwrap();
    buffer[xy_to_idx(start_x + 0, start_y + 1)] = p[seq[3]].to_color_sequence()[n].to_string().bytes().next().unwrap();
    buffer[xy_to_idx(start_x + 1, start_y + 1)] = center_color.to_string().bytes().next().unwrap();
    buffer[xy_to_idx(start_x + 2, start_y + 1)] = p[seq[4]].to_color_sequence()[n].to_string().bytes().next().unwrap();
    buffer[xy_to_idx(start_x + 0, start_y + 2)] = p[seq[5]].to_color_sequence()[n].to_string().bytes().next().unwrap();
    buffer[xy_to_idx(start_x + 1, start_y + 2)] = p[seq[6]].to_color_sequence()[n].to_string().bytes().next().unwrap();
    buffer[xy_to_idx(start_x + 2, start_y + 2)] = p[seq[7]].to_color_sequence()[n].to_string().bytes().next().unwrap();

}
