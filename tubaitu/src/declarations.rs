use std::{fmt::Display, ops::Deref};

use crate::*;

#[derive(Debug, Clone, Copy, Hash, Default)]
pub struct Cube2 {
    pub pieces: [Piece; 8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Piece {
    pub rotation: PieceRotation,
}

/// Stored as [top color][front color], which uniquely defines a rotation (because the cross product isn't commutative!)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PieceRotation {
    WR, WO, WG,
    RW, BW, OW, GW,
    YR, YB, YO, YG,
    RY, BY, OY, GY,
    OG, GO, OB, BO,
    RG, GR, RB, BR,
    #[default] WB,
}

impl Cube2 {
    pub fn make_move(&mut self, m: &Move) {
        match m.side {
            MoveSide::R => cycle_face(&mut self.pieces, FACE_RIGHT_SEQ_CYCLE, m),
            MoveSide::L => cycle_face(&mut self.pieces, FACE_LEFT_SEQ_CYCLE , m),
            MoveSide::U => cycle_face(&mut self.pieces, FACE_UP_SEQ_CYCLE   , m),
            MoveSide::B => cycle_face(&mut self.pieces, FACE_BACK_SEQ_CYCLE , m),
            MoveSide::F => cycle_face(&mut self.pieces, FACE_FRONT_SEQ_CYCLE, m),
            MoveSide::D => cycle_face(&mut self.pieces, FACE_DOWN_SEQ_CYCLE , m),
        };
    }

    pub fn scramble(scramble: &MoveSeq) -> Self {
	let mut c = Cube2::default();
	for m in &scramble.0 {
	    c.make_move(&m);
	}
	c
    }
    pub fn random_scramble(length: usize) -> (Self, MoveSeq) {
	use rand::Rng;
	fn get_move_from_n(n: usize) -> Move {
	    match n {
		0 => Move::new("R"), 1 => Move::new("L"), 2 => Move::new("R"), 3 => Move::new("B"),
		4 => Move::new("U"), 5 => Move::new("D"), 6 => Move::new("R'"), 7 => Move::new("L'"),
		8 => Move::new("R'"), 9 => Move::new("B'"), 10 => Move::new("U'"), 11 => Move::new("D'"),
		_ => unreachable!("Range reaches 12")
	    }
	}

	let mut scramble = vec![];

	let mut c = Cube2::default();
	for _ in 0..length {
	    let mov = get_move_from_n(rand::thread_rng().gen_range(0..12));
	    scramble.push(mov);
	    c.make_move(&mov);
	}

	(c, scramble.into())
    }
 
}

impl std::cmp::PartialEq for Cube2 {
    fn eq(&self, other: &Self) -> bool {

        for o in &get_orientation_generators() {
            for r in &get_rotation_generators() {
                let mut alternate_cube = self.clone();
                for m1 in o { alternate_cube.make_move(m1) }
                for m2 in r { alternate_cube.make_move(m2) }
                if alternate_cube.pieces == other.pieces { return true; }
            }
        }

        return false;
    }
 }
impl std::cmp::Eq for Cube2 { }


// If you touch these, remember to change the magic numbers in Cube's Display impl!
pub const CUBE_PRINT_WIDTH: usize = 2*4 + 5 + 1;
pub const CUBE_PRINT_HEIGHT: usize = 2*3 + 3 + 1;
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

impl std::fmt::Display for Cube2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
	let mut buffer: [u8; CUBE_PRINT_WIDTH*CUBE_PRINT_HEIGHT] =
	    [' ' as u8; CUBE_PRINT_WIDTH*CUBE_PRINT_HEIGHT];

	// Newlines
	for y in 0..CUBE_PRINT_HEIGHT - 1 { buffer[xy_to_idx(CUBE_PRINT_WIDTH - 1, y)] = '\n' as u8 }

	// Horizontals
	for y in [3, 6] {
	    for x in 0..CUBE_PRINT_WIDTH - 1 { buffer[xy_to_idx(x, y)] = CUBE_PRINT_HORIZ_DIVIDER_TMP } 
	}
	for y in [0, CUBE_PRINT_HEIGHT - 1] {
	    for x in 4..=6 - 1 { buffer[xy_to_idx(x, y)] = CUBE_PRINT_HORIZ_DIVIDER_TMP } 
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

        print_add_face(&mut buffer, &self.pieces, 2, FACE_UP_SEQ_PRINT,    4,  1);
        print_add_face(&mut buffer, &self.pieces, 3, FACE_LEFT_SEQ_PRINT,  1,  4);
        print_add_face(&mut buffer, &self.pieces, 1, FACE_FRONT_SEQ_PRINT, 4,  4);
        print_add_face(&mut buffer, &self.pieces, 0, FACE_RIGHT_SEQ_PRINT, 7,  4);
        print_add_face(&mut buffer, &self.pieces, 4, FACE_BACK_SEQ_PRINT,  10, 4);
        print_add_face(&mut buffer, &self.pieces, 5, FACE_DOWN_SEQ_PRINT,  4,  7);

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

        write!(f, "{}", s)
    }

}

fn print_add_face(
    buffer: &mut [u8; CUBE_PRINT_WIDTH*CUBE_PRINT_HEIGHT],
    p: &[Piece; 8],
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

pub fn reverse_seq([a, b, c, d]: [usize; 4]) -> [usize; 4] {
    [d, c, b, a]
}

pub fn cycle_face(face: &mut [Piece; 8], mut face_seq: [usize; 4], mov @ Move { side: _, prime }: &Move) {
    if *prime { face_seq = reverse_seq(face_seq); }

    // Move the pieces
    cycle_items(face, face_seq); 

    // Rotate the pieces
    for i in face_seq {
	face[i].rotate(mov)
    }
}



pub fn cycle_items<T: Clone, const N: usize>(v: &mut [T; N], idxs: [usize; 4]) {
    cycle_items_unchecked(v, idxs);
}

pub fn cycle_items_safe<T: Clone, const N: usize>(v: &mut [T; N], idxs: [usize; 4]) {
    v.swap(idxs[0], idxs[1]);
    v.swap(idxs[0], idxs[2]);
    v.swap(idxs[0], idxs[3]);
}
pub fn cycle_items_old<T: Clone, const N: usize>(v: &mut [T; N], idxs: [usize; 4]) {
    let e = v[idxs[3]].clone();

    v[idxs[3]] = v[idxs[2]].clone();
    v[idxs[2]] = v[idxs[1]].clone();
    v[idxs[1]] = v[idxs[0]].clone();
    v[idxs[0]] = e;
}

pub fn cycle_items_unchecked<T: Clone, const N: usize>(v: &mut [T; N], idxs: [usize; 4]) {
    use std::ptr;
    unsafe { ptr::swap(
	&mut v[idxs[0]] as *mut T,
	&mut v[idxs[1]] as *mut T,
    )};
    unsafe { ptr::swap(
	&mut v[idxs[0]] as *mut T,
	&mut v[idxs[2]] as *mut T,
    )};
    unsafe { ptr::swap(
	&mut v[idxs[0]] as *mut T,
	&mut v[idxs[3]] as *mut T,
    )};
}
