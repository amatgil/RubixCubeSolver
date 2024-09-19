use crate::*;

// TODO: Make sure the implementation of Hash is coherent with the manual one of PartialEq
#[derive(Debug, Clone, Copy, Hash, Default)]
pub struct Cube2 {
    pub pieces: [Piece; 8],
}


fn get_orientation_generators() -> [Vec<Move>; 6] {
    [
	vec![],
	vec![Move::F, Move::B],
	vec![Move::R, Move::L],
	vec![Move::F, Move::B],
	vec![Move::R, Move::L],
	vec![Move::F, Move::B, Move::F, Move::B],
    ]
}

fn get_rotation_generators() -> [Vec<Move>; 4] {
    [
	vec![],
	vec![Move::U, Move::D],
	vec![Move::D, Move::U],
	vec![Move::U, Move::D, Move::U, Move::D],
    ]
}

impl std::cmp::PartialEq for Cube2 {
    fn eq(&self, other: &Self) -> bool {
        for o in &get_orientation_generators() {
            for r in &get_rotation_generators() {
                let mut alternate_cube = *self;
                for m1 in o { alternate_cube.make_move(*m1) }
                for m2 in r { alternate_cube.make_move(*m2) }
                if alternate_cube.pieces == other.pieces { return true; }
            }
        }
        false
    }
 }
impl std::cmp::Eq for Cube2 { }

fn xy_to_idx(x: usize, y: usize) -> usize { y*CUBE_PRINT_WIDTH + x }

// If you touch these, remember to change the magic numbers in Cube2's Display impl!
pub const CUBE_PRINT_WIDTH: usize = 2*4 + 5 + 1;
pub const CUBE_PRINT_HEIGHT: usize = 2*3 + 3 + 1;
const CUBE_PRINT_HORIZ_DIVIDER_TMP: u8 = b'-';
const CUBE_PRINT_VERTI_DIVIDER_TMP: u8 = b'|';
const CUBE_PRINT_CROSS_DIVIDER_TMP: u8 = b'+';
const CUBE_PRINT_TOP_L_DIVIDER_TMP: u8 = b'1';
const CUBE_PRINT_TOP_R_DIVIDER_TMP: u8 = b'2';
const CUBE_PRINT_BOT_L_DIVIDER_TMP: u8 = b'3';
const CUBE_PRINT_BOT_R_DIVIDER_TMP: u8 = b'4';
const CUBE_PRINT_NORMT_DIVIDER_TMP: u8 = b'5';
const CUBE_PRINT_UPSDT_DIVIDER_TMP: u8 = b'6';

const CUBE_PRINT_HORIZ_DIVIDER: char = '━';
const CUBE_PRINT_VERTI_DIVIDER: char = '┃';
const CUBE_PRINT_CROSS_DIVIDER: char = '╋';
const CUBE_PRINT_TOP_L_DIVIDER: char = '┏';
const CUBE_PRINT_TOP_R_DIVIDER: char = '┓';
const CUBE_PRINT_BOT_L_DIVIDER: char = '┗';
const CUBE_PRINT_BOT_R_DIVIDER: char = '┛';
const CUBE_PRINT_NORMT_DIVIDER: char = '┳';
const CUBE_PRINT_UPSDT_DIVIDER: char = '┻';

fn print_add_face(
    buffer: &mut [u8; CUBE_PRINT_WIDTH*CUBE_PRINT_HEIGHT],
    p: [Piece; 8],
    n: usize,
    seq: [usize; 4],
    start_x: usize,
    start_y: usize,
) {
    let (mut x, mut y) = (start_x, start_y);
    for (i, v) in seq.into_iter().enumerate() {
	let cols = p[v].to_color_sequence();
	let buffer_idx = y*CUBE_PRINT_WIDTH + x;
	buffer[buffer_idx] = cols[n].to_string().bytes().next().unwrap();

	x += 1;
	if i == 1 {
	    x = start_x;
	    y += 1;
	}
    }
}

impl std::fmt::Display for Cube2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
	let mut buffer: [u8; CUBE_PRINT_WIDTH*CUBE_PRINT_HEIGHT] =
	    [b' '; CUBE_PRINT_WIDTH*CUBE_PRINT_HEIGHT];

	// Newlines
	for y in 0..CUBE_PRINT_HEIGHT - 1 { buffer[xy_to_idx(CUBE_PRINT_WIDTH - 1, y)] = b'\n' }

	// Horizontals
	for y in [3, 6] {
	    for x in 0..CUBE_PRINT_WIDTH - 1 { buffer[xy_to_idx(x, y)] = CUBE_PRINT_HORIZ_DIVIDER_TMP } 
	}
	for y in [0, CUBE_PRINT_HEIGHT - 1] {
	    for x in 4..6  { buffer[xy_to_idx(x, y)] = CUBE_PRINT_HORIZ_DIVIDER_TMP } 
	}

	// Verticals
	for x in [0, 3, 6, 9, 12] {
	    for y in 3..6 { buffer[xy_to_idx(x, y)] = CUBE_PRINT_VERTI_DIVIDER_TMP; }
	}

	for x in [3, 6] {
	    for y in 1..CUBE_PRINT_HEIGHT - 1 {
		buffer[xy_to_idx(x, y)] = CUBE_PRINT_VERTI_DIVIDER_TMP;
	    }
	}

	for (x, y) in [(0, 3), (3, 0)]  { buffer[xy_to_idx(x, y)] = CUBE_PRINT_TOP_L_DIVIDER_TMP; }
	for (x, y) in [(12, 3), (6, 0)] { buffer[xy_to_idx(x, y)] = CUBE_PRINT_TOP_R_DIVIDER_TMP; }
	for (x, y) in [(0, 6), (3, 9)]  { buffer[xy_to_idx(x, y)] = CUBE_PRINT_BOT_L_DIVIDER_TMP; }
	for (x, y) in [(6, 9), (12, 6)] { buffer[xy_to_idx(x, y)] = CUBE_PRINT_BOT_R_DIVIDER_TMP; }

	
	buffer[xy_to_idx(9, 3)] = CUBE_PRINT_NORMT_DIVIDER_TMP; 
	buffer[xy_to_idx(9, 6)] = CUBE_PRINT_UPSDT_DIVIDER_TMP;

	for x in [3, 6] {
	    for y in [3, 6] { buffer[xy_to_idx(x, y)] = CUBE_PRINT_CROSS_DIVIDER_TMP; }
	}

        print_add_face(&mut buffer, self.pieces, 2, FACE_UP_SEQ_PRINT,    4,  1);
        print_add_face(&mut buffer, self.pieces, 3, FACE_LEFT_SEQ_PRINT,  1,  4);
        print_add_face(&mut buffer, self.pieces, 1, FACE_FRONT_SEQ_PRINT, 4,  4);
        print_add_face(&mut buffer, self.pieces, 0, FACE_RIGHT_SEQ_PRINT, 7,  4);
        print_add_face(&mut buffer, self.pieces, 4, FACE_BACK_SEQ_PRINT,  10, 4);
        print_add_face(&mut buffer, self.pieces, 5, FACE_DOWN_SEQ_PRINT,  4,  7);

	let s = std::str::from_utf8(&buffer).expect("invalid utf-8 sequence (should be impossible)");

	let s: String = s.chars().map(|c| {
	    match c as u8 {
		CUBE_PRINT_VERTI_DIVIDER_TMP => CUBE_PRINT_VERTI_DIVIDER,
		CUBE_PRINT_HORIZ_DIVIDER_TMP => CUBE_PRINT_HORIZ_DIVIDER,
		CUBE_PRINT_CROSS_DIVIDER_TMP => CUBE_PRINT_CROSS_DIVIDER,
		CUBE_PRINT_TOP_L_DIVIDER_TMP => CUBE_PRINT_TOP_L_DIVIDER,
		CUBE_PRINT_TOP_R_DIVIDER_TMP => CUBE_PRINT_TOP_R_DIVIDER,
		CUBE_PRINT_BOT_L_DIVIDER_TMP => CUBE_PRINT_BOT_L_DIVIDER,
		CUBE_PRINT_BOT_R_DIVIDER_TMP => CUBE_PRINT_BOT_R_DIVIDER,
		CUBE_PRINT_NORMT_DIVIDER_TMP => CUBE_PRINT_NORMT_DIVIDER,
		CUBE_PRINT_UPSDT_DIVIDER_TMP => CUBE_PRINT_UPSDT_DIVIDER,
		_ => c
	    }
	}).collect();

        write!(f, "{s}")
    }

}
