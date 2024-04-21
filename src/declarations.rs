use crate::*;

#[derive(Debug, Clone, Copy, Hash, Default)]
pub struct Cube {
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

impl Cube {
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

    pub fn scramble(scramble: &Vec<Move>) -> Self {
	let mut c = Cube::default();
	for m in scramble {
	    c.make_move(m);
	}
	c
    }
}

#[test]
fn redundant_move_right() {
    let mut cube = Cube::default();
    cube.make_move(&Move::new("R"));
    cube.make_move(&Move::new("R"));
    cube.make_move(&Move::new("R"));
    cube.make_move(&Move::new("R"));
    assert_eq!(cube, Cube::default());
}

#[test]
fn redundant_move_up() {
    let mut cube = Cube::default();
    cube.make_move(&Move::new("U"));
    cube.make_move(&Move::new("U"));
    cube.make_move(&Move::new("U"));
    cube.make_move(&Move::new("U"));
    assert_eq!(cube, Cube::default());
}

#[test]
fn redundant_move_double_up() {
    let mut cube = Cube::default();
    cube.make_move(&Move::new("U"));
    cube.make_move(&Move::new("U"));
    cube.make_move(&Move::new("U'"));
    cube.make_move(&Move::new("U'"));
    assert_eq!(cube, Cube::default());
}

impl std::cmp::PartialEq for Cube {
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
impl std::cmp::Eq for Cube { }


pub const CUBE_PRINT_WIDTH: usize = 16 + 1;
pub const CUBE_PRINT_HEIGHT: usize = 6;
impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
	let mut buffer: [u8; CUBE_PRINT_WIDTH*CUBE_PRINT_HEIGHT] =
	    [' ' as u8; CUBE_PRINT_WIDTH*CUBE_PRINT_HEIGHT];

	for y in 0..CUBE_PRINT_HEIGHT - 1 {
	    buffer[y*CUBE_PRINT_WIDTH + CUBE_PRINT_WIDTH - 1] = '\n' as u8;
	}

        print_add_face(&mut buffer, &self.pieces, 5, FACE_DOWN_SEQ_PRINT,  2, 4);
        print_add_face(&mut buffer, &self.pieces, 4, FACE_BACK_SEQ_PRINT,  6, 2);
        print_add_face(&mut buffer, &self.pieces, 0, FACE_RIGHT_SEQ_PRINT, 4, 2);
        print_add_face(&mut buffer, &self.pieces, 1, FACE_FRONT_SEQ_PRINT, 2, 2);
        print_add_face(&mut buffer, &self.pieces, 3, FACE_LEFT_SEQ_PRINT,  0, 2);
        print_add_face(&mut buffer, &self.pieces, 2, FACE_UP_SEQ_PRINT,    2, 0);

	let s = std::str::from_utf8(&buffer).expect("invalid utf-8 sequence (should be impossible)");
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
