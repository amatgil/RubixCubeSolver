use crate::*;

#[derive(Debug, Clone, Copy, Hash)]
pub struct Cube {
    pub pieces: [Piece; 8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece{
    pub cols: [Color; 6],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color { White, Red, Blue, Yellow, Orange, Green }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub side: MoveSide,
    pub prime: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveSide { R, F, U, L, B, D }

impl Move {
    pub fn new(s: &str) -> Move {
        let ms = s.chars().nth(0).unwrap();
        let k = s.chars().nth(1).is_some();

        let m = match ms {
            'R' => MoveSide::R,
            'F' => MoveSide::F,
            'U' => MoveSide::U,
            'L' => MoveSide::L,
            'B' => MoveSide::B,
            'D' => MoveSide::D,
            _ => panic!("{ms} is not a valid face move"),
        };

        Move {
            side: m,
            prime: k,
        }
    }
}

impl Piece {
    pub fn new(inp: [char; 6]) -> Piece {
        let cols = inp.into_iter().map(|c| Color::new(c)).collect::<Vec<_>>().try_into().unwrap();
        Piece { cols }
    }
}
fn reverse_seq([a, b, c, d]: [usize; 4]) -> [usize; 4] {
    [d, c, b, a]
}
fn cycle_face(face: &mut [Piece; 8], mut face_seq: [usize; 4], mut col_seq: [usize; 4], prime: bool) {
    if prime {
        face_seq = reverse_seq(face_seq);
        col_seq = reverse_seq(col_seq);
    }
    cycle_items(face, face_seq);
    for i in face_seq {
        cycle_items(&mut face[i].cols, col_seq);
    }
}

impl Cube {
    pub fn make_move(&mut self, m: &Move) {
        match m.side {
            MoveSide::R => cycle_face(&mut self.pieces, FACE_RIGHT_SEQ_CYCLE , COLOR_RIGHT_SEQ , m.prime),
            MoveSide::L => cycle_face(&mut self.pieces, FACE_LEFT_SEQ_CYCLE , COLOR_LEFT_SEQ , m.prime),
            MoveSide::U => cycle_face(&mut self.pieces, FACE_UP_SEQ_CYCLE   , COLOR_UP_SEQ   , m.prime),
            MoveSide::B => cycle_face(&mut self.pieces, FACE_BACK_SEQ_CYCLE , COLOR_BACK_SEQ , m.prime),
            MoveSide::F => cycle_face(&mut self.pieces, FACE_FRONT_SEQ_CYCLE, COLOR_FRONT_SEQ, m.prime),
            MoveSide::D => cycle_face(&mut self.pieces, FACE_DOWN_SEQ_CYCLE , COLOR_DOWN_SEQ , m.prime),
        };
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
fn print_add_face(out: &mut String, p: &[Piece; 8], s: &str, n: usize, seq: [usize; 4]) {
    out.push_str("\n---\n");
    out.push_str(&format!("{s}: \n"));
    for (i, v) in seq.into_iter().enumerate() {
        if i == 2 { out.push('\n') }
        out.push_str(&p[v].cols[n].to_string());
    }
    out.push_str("\n---\n");

}
impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut out = String::new();

        // Right
        print_add_face(&mut out, &self.pieces, "RIGHT", 0, FACE_RIGHT_SEQ_PRINT);
        print_add_face(&mut out, &self.pieces, "FRONT", 1, FACE_FRONT_SEQ_PRINT);
        print_add_face(&mut out, &self.pieces, "UP", 2, FACE_UP_SEQ_PRINT);
        print_add_face(&mut out, &self.pieces, "LEFT", 3, FACE_LEFT_SEQ_PRINT);
        print_add_face(&mut out, &self.pieces, "BACK", 4, FACE_BACK_SEQ_PRINT);
        print_add_face(&mut out, &self.pieces, "DOWN", 5, FACE_DOWN_SEQ_PRINT);

        write!(f, "{}\n\n", out)
    }

}

impl Color {
    pub fn new(c: char) -> Color {
        use Color as C;
        match c.to_ascii_uppercase() {
            'W' => C::White,
            'R' => C::Red,
            'B' => C::Blue,
            'Y' => C::Yellow,
            'O' => C::Orange,
            'G' => C::Green,
            _ => panic!("Char {c} is not a valid color"),
        }
    }
}

impl std::cmp::PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        let orientation_generators = [
            vec![],
            vec![Move::new("F") , Move::new("B")],
            vec![Move::new("R"), Move::new("L'")],
            vec![Move::new("F"), Move::new("B'")],
            vec![Move::new("R'"), Move::new("L'")],
            vec![Move::new("F'"), Move::new("B"), Move::new("F'"), Move::new("B")],
        ];

        let rotation_generators = [
            vec![],
            vec![Move::new("U"), Move::new("D'")],
            vec![Move::new("D"), Move::new("U'")],
            vec![Move::new("U"), Move::new("D'"), Move::new("U"), Move::new("D'")],
        ];

        for o in &orientation_generators {
            for r in &rotation_generators {
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