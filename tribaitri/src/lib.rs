use std::fmt::Display;

use shared::*;
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
/// Internally, it is represented by the 6 edges and 14 corners only. Their indicies are defined by the corresponding ThreeByCorner and ThreeByEdge enums.
/// Note that equality is direct here, unlike a 2x2: there are no symmetries to speak of because the centers ground us
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Cube3 {
    pieces: [Piece; 3*3*3 - 1 - 6], // 20: edges + corners
}


impl Default for Cube3 {
    fn default() -> Self {

        Self{ pieces: [Piece::default(); 20] }
    }
}
    
impl Solvable for Cube3 {
    fn moves_of_adjacency() -> Vec<Move> {
        Vec::from([
            Move::new("R"), Move::new("R'"),
            Move::new("L"), Move::new("L'"),
            Move::new("F"), Move::new("F'"),
            Move::new("B"), Move::new("B'"),
            Move::new("U"), Move::new("U'"),
            Move::new("D"), Move::new("D'")
        ])
    }
    fn make_move(&mut self, m: &Move) {
        make_three_by_three_move(self, m);
    }
}


pub const CUBE_PRINT_WIDTH: usize = 3*4 + 5 + 1;
pub const CUBE_PRINT_HEIGHT: usize = 3*3 + 3 + 1;
const CUBE_PRINT_HORIZ_DIVIDER_TMP: u8 = '-' as u8;
const CUBE_PRINT_VERTI_DIVIDER_TMP: u8 = '|' as u8;
const CUBE_PRINT_CROSS_DIVIDER_TMP: u8 = '+' as u8;
const CUBE_PRINT_TOP_L_DIVIDER_TMP: u8 = '1' as u8;
const CUBE_PRINT_TOP_R_DIVIDER_TMP: u8 = '2' as u8;
const CUBE_PRINT_BOT_L_DIVIDER_TMP: u8 = '3' as u8;
const CUBE_PRINT_BOT_R_DIVIDER_TMP: u8 = '4' as u8;
const CUBE_PRINT_NORMT_DIVIDER_TMP: u8 = '5' as u8;
const CUBE_PRINT_UPSDT_DIVIDER_TMP: u8 = '6' as u8;

const CUBE_PRINT_HORIZ_DIVIDER: char = '━';
const CUBE_PRINT_VERTI_DIVIDER: char = '┃';
const CUBE_PRINT_CROSS_DIVIDER: char = '╋';
const CUBE_PRINT_TOP_L_DIVIDER: char = '┏';
const CUBE_PRINT_TOP_R_DIVIDER: char = '┓';
const CUBE_PRINT_BOT_L_DIVIDER: char = '┗';
const CUBE_PRINT_BOT_R_DIVIDER: char = '┛';
const CUBE_PRINT_NORMT_DIVIDER: char = '┳';
const CUBE_PRINT_UPSDT_DIVIDER: char = '┻';
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
	    [' ' as u8; CUBE_PRINT_WIDTH*CUBE_PRINT_HEIGHT];


	for y in 0..CUBE_PRINT_HEIGHT - 1 { buffer[xy_to_idx(CUBE_PRINT_WIDTH - 1, y)] = '\n' as u8 }
	for y in [4, 8] {
	    for x in 0..CUBE_PRINT_WIDTH - 1 { buffer[xy_to_idx(x, y)] = '-' as u8 } 
	}

	for y in [0, CUBE_PRINT_HEIGHT - 1] {
	    for x in 5..=8 - 1 { buffer[xy_to_idx(x, y)] = '-' as u8 } 
	}

        print_add_face(&mut buffer, &self.pieces, 3, LEFT_PRINTING_CYCLE,  1,  5, Color::Green);
        print_add_face(&mut buffer, &self.pieces, 4, BACK_PRINTING_CYCLE,  13, 5, Color::Red);
        print_add_face(&mut buffer, &self.pieces, 1, FRONT_PRINTING_CYCLE,  5,  5, Color::Orange);
        print_add_face(&mut buffer, &self.pieces, 2, UP_PRINTING_CYCLE,  5,  1, Color::Yellow);
        print_add_face(&mut buffer, &self.pieces, 0, RIGHT_PRINTING_CYCLE,  9,  5, Color::Blue);
        print_add_face(&mut buffer, &self.pieces, 5, DOWN_PRINTING_CYCLE,  5,  9, Color::White);

	let s = std::str::from_utf8(&buffer).expect("invalid utf-8 sequence (should be impossible)");
        write!(f, "{}", s)
    }
}

fn print_add_face(
    buffer: &mut [u8; CUBE_PRINT_WIDTH*CUBE_PRINT_HEIGHT],
    p: &[Piece; 20],
    n: usize,
    seq: [usize; 8],
    start_x: usize,
    start_y: usize,
    center_color: Color
) {
    let (mut x, mut y) = (start_x, start_y);
    let mut iter = seq.into_iter().enumerate();
    for (i, v) in iter.clone().take(4) {
	let cols = p[v].to_color_sequence();
	let buffer_idx = y*CUBE_PRINT_WIDTH + x;
        buffer[buffer_idx] = cols[n].to_string().bytes().next().unwrap();

	x += 1;
	if i == 2 {
	    x = start_x;
	    y += 1;
	}
    }

    // Center
    buffer[xy_to_idx(start_x + 1, start_y + 1)] = center_color.to_string().bytes().next().unwrap();

    // Right of center
    buffer[xy_to_idx(start_x + 2, start_y + 1)] = p[4].to_color_sequence()[n].to_string().bytes().next().unwrap();
    buffer[xy_to_idx(start_x + 0, start_y + 2)] = p[5].to_color_sequence()[n].to_string().bytes().next().unwrap();
    buffer[xy_to_idx(start_x + 1, start_y + 2)] = p[6].to_color_sequence()[n].to_string().bytes().next().unwrap();
    buffer[xy_to_idx(start_x + 2, start_y + 2)] = p[7].to_color_sequence()[n].to_string().bytes().next().unwrap();
}
