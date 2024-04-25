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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Color { #[default] White, Red, Blue, Yellow, Orange, Green }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub side: MoveSide,
    pub prime: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveSide { R, F, U, L, B, D }

impl Move {
    pub fn new(s: &str) -> Move {
	if s.len() > 2 { panic!("{s} no és un moviment legal"); } 
        let ms = s.chars().nth(0).unwrap();
        let k = s.chars().nth(1);
	if let Some(prima) = k {
	    if prima != '\'' { panic!("{s} té un segon char que no és una prima") }
	}

        let m = match ms {
            'R' => MoveSide::R,
            'F' => MoveSide::F,
            'U' => MoveSide::U,
            'L' => MoveSide::L,
            'B' => MoveSide::B,
            'D' => MoveSide::D,
            _ => panic!("{ms} is not a valid face move"),
        };

        Move { side: m, prime: k.is_some() }
    }
    pub fn opposite(&self) -> Self {
	Self { prime: !self.prime, ..*self }

    }
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

#[derive(Debug, Clone)]
pub struct MoveSeq(pub Vec<Move>);

impl MoveSeq {
    pub fn reversed(&self) -> Self {
        Self(self.0.iter().rev().map(|m| m.opposite()).collect())
    }
}

impl From<Vec<Move>> for MoveSeq { fn from(value: Vec<Move>) -> Self { Self(value) } }

impl Deref for MoveSeq {
    type Target = Vec<Move>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl IntoIterator for MoveSeq {
    type Item = Move;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExpandedMove {
    L { prime: bool }, L2,
    R { prime: bool }, R2,
    F { prime: bool }, F2,
    B { prime: bool }, B2,
    U { prime: bool }, U2,
    D { prime: bool }, D2,
    Nothing
}
impl ExpandedMove {
    /// This preserves rotations: so R L *could* be rewritten as 2L or 2R while preserving equality but, since it does not preserve rotation, it will not be compressed
    fn compress(a: Self, b: Self) -> Option<Self> {
	match (a, b) {
	    (Self::Nothing, _)
		| (_, Self::Nothing)
		=> None,
	    (Self::L { prime: p1 }, Self::L { prime: p2 } ) 
		| (Self::R { prime: p1 }, Self::R { prime: p2 } )
		| (Self::F { prime: p1 }, Self::F { prime: p2 } )
		| (Self::B { prime: p1 }, Self::B { prime: p2 } )
		| (Self::B { prime: p1 }, Self::B { prime: p2 } ) if p1 != p2
		=> Some(Self::Nothing),
	    (Self::L2, Self::L2)
		| (Self::R2, Self::R2)
		| (Self::F2, Self::D2)
		| (Self::B2, Self::B2)
		| (Self::U2, Self::U2)
		| (Self::D2, Self::D2)
		=> Some(Self::Nothing),
	    (Self::L { prime: p1 }, Self::L { prime: p2} ) if p1 == p2 => Some(Self::L2),
	    (Self::R { prime: p1 }, Self::R { prime: p2} ) if p1 == p2 => Some(Self::R2),
	    (Self::F { prime: p1 }, Self::F { prime: p2} ) if p1 == p2 => Some(Self::F2),
	    (Self::B { prime: p1 }, Self::B { prime: p2} ) if p1 == p2 => Some(Self::B2),
	    (Self::U { prime: p1 }, Self::U { prime: p2} ) if p1 == p2 => Some(Self::U2),
	    (Self::D { prime: p1 }, Self::D { prime: p2} ) if p1 == p2 => Some(Self::D2),
	    (Self::L { prime: p1 }, Self::L2) => Some(Self::L { prime: !p1 }),
	    (Self::R { prime: p1 }, Self::R2) => Some(Self::R { prime: !p1 }),
	    (Self::F { prime: p1 }, Self::F2) => Some(Self::F { prime: !p1 }),
	    (Self::B { prime: p1 }, Self::B2) => Some(Self::B { prime: !p1 }),
	    (Self::U { prime: p1 }, Self::U2) => Some(Self::U { prime: !p1 }),
	    (Self::D { prime: p1 }, Self::D2) => Some(Self::D { prime: !p1 }),
	    (Self::L2, Self::L { prime: p1 }) => Some(Self::L { prime: !p1 }),
	    (Self::R2, Self::R { prime: p1 }) => Some(Self::R { prime: !p1 }),
	    (Self::F2, Self::F { prime: p1 }) => Some(Self::F { prime: !p1 }),
	    (Self::B2, Self::B { prime: p1 }) => Some(Self::B { prime: !p1 }),
	    (Self::U2, Self::U { prime: p1 }) => Some(Self::U { prime: !p1 }),
	    (Self::D2, Self::D { prime: p1 }) => Some(Self::D { prime: !p1 }),
	    _ => None
	}
    }
} 

impl Display for ExpandedMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	let is_p = |&b: &bool| if b { "'" } else { "" };
	let o: String = match self {
	    ExpandedMove::L { prime } => format!("L{}", is_p(prime)),
	    ExpandedMove::R { prime } => format!("R{}", is_p(prime)),
	    ExpandedMove::F { prime } => format!("F{}", is_p(prime)),
	    ExpandedMove::B { prime } => format!("B{}", is_p(prime)),
	    ExpandedMove::U { prime } => format!("U{}", is_p(prime)),
	    ExpandedMove::D { prime } => format!("D{}", is_p(prime)),
	    ExpandedMove::L2 => "L2".into(),
	    ExpandedMove::R2 => "R2".into(),
	    ExpandedMove::F2 => "F2".into(),
	    ExpandedMove::B2 => "B2".into(),
	    ExpandedMove::U2 => "U2".into(),
	    ExpandedMove::D2 => "D2".into(),
	    ExpandedMove::Nothing => "".into(),
	};
	write!(f, "{o}")
    }
}

impl Display for MoveSeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	let mut stack: Vec<ExpandedMove> = vec![];
	for mov in self.0.iter() {
	    let ext = match mov {
		Move { side: MoveSide::L , prime } => ExpandedMove::L { prime: *prime },
		Move { side: MoveSide::R , prime } => ExpandedMove::R { prime: *prime },
		Move { side: MoveSide::F , prime } => ExpandedMove::F { prime: *prime },
		Move { side: MoveSide::B , prime } => ExpandedMove::B { prime: *prime },
		Move { side: MoveSide::U , prime } => ExpandedMove::U { prime: *prime },
		Move { side: MoveSide::D , prime } => ExpandedMove::D { prime: *prime },
	    };
	    stack.push(ext);
	    while stack.len() > 1 {
		if let Some(c) = ExpandedMove::compress(
		    stack[stack.len() - 2],
		    stack[stack.len() - 1],
		) {
		    stack.pop();
		    stack.pop();
		    stack.push(c);
		} else { break; }
	    }
	}
	let mut o = String::new();
	for m in stack {
	    if m != ExpandedMove::Nothing { o.push_str(&format!("{} ", m)) };
	}

	write!(f, "[ {o} ]")
    }
}

//#[test]
//fn moveseq_debuggign() {
//    let (_, random_moves) = Cube::random_scramble(6); 
//
//    let random_moves = vec![
//	Move::new("R"),
//	Move::new("R"),
//	Move::new("R"),
//	Move::new("R"),
//	Move::new("R"),
//	Move::new("R"),
//	Move::new("L"),
//	Move::new("L"),
//	Move::new("L"),
//	Move::new("R"),
//	Move::new("L"),
//	Move::new("R"),
//	Move::new("L"),
//    ];
//    print_solution(&random_moves);
//    let seq: MoveSeq = random_moves.into();
//    println!("Compressed: {seq}");
//
//    panic!()
//}

#[test]
fn redundant_move_right() {
    let mut cube = Cube2::default();
    cube.make_move(&Move::new("R"));
    cube.make_move(&Move::new("R"));
    cube.make_move(&Move::new("R"));
    cube.make_move(&Move::new("R"));
    assert_eq!(cube, Cube2::default());
}

#[test]
fn redundant_move_up() {
    let mut cube = Cube2::default();
    cube.make_move(&Move::new("U"));
    cube.make_move(&Move::new("U"));
    cube.make_move(&Move::new("U"));
    cube.make_move(&Move::new("U"));
    assert_eq!(cube, Cube2::default());
}

#[test]
fn redundant_move_double_up() {
    let mut cube = Cube2::default();
    cube.make_move(&Move::new("U"));
    cube.make_move(&Move::new("U"));
    cube.make_move(&Move::new("U'"));
    cube.make_move(&Move::new("U'"));
    assert_eq!(cube, Cube2::default());
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

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let o = match self {
            Color::White  => "W",
            Color::Red    => "R",
            Color::Blue   => "B",
            Color::Yellow => "Y",
            Color::Orange => "O",
            Color::Green  => "G",
        };

        write!(f, "{o}")
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut out = String::new();
        out.push(match self.side {
            MoveSide::R => 'R',
            MoveSide::F => 'F',
            MoveSide::U => 'U',
            MoveSide::L => 'L',
            MoveSide::B => 'B',
            MoveSide::D => 'D',
        });
        if self.prime { out.push('\'') }
    
        write!(f, "{out}")
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

#[test]
fn cycling_test() {
    let t1 = [1, 2, 3, 4, 5];
    let idx = [0, 1, 2, 3];

    let mut a = t1.clone();
    let mut b = t1.clone();
    cycle_items(&mut a, idx);
    cycle_items_old(&mut b, idx);

    assert_eq!(a, b);
}

#[test]
fn cycling_test_unchecked() {
    let t1 = [1, 2, 3, 4, 5];
    let idx = [0, 1, 2, 3];

    let mut a = t1.clone();
    let mut b = t1.clone();
    cycle_items_safe(&mut a, idx);
    cycle_items_unchecked(&mut b, idx);

    assert_eq!(a, b);
}
