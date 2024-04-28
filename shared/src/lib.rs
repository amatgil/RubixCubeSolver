use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fmt::Display,
    hash::Hash,
    ops::Deref,
    rc::Rc,
    time::Instant,
};

pub mod colors;
pub use colors::*;

pub mod ui;
pub use ui::*;

pub const FLOAT_EPSILON: f64 = 0.0001;

struct State<C> {
    past_moves: Vec<Move>,
    cube: C,
}
impl<C: Solvable> Hash for State<C> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cube.hash(state);
    }
}
impl<C: Solvable> PartialEq for State<C> {
    fn eq(&self, rhs: &Self) -> bool {
        self.cube == rhs.cube
    }
}
impl<C: Solvable> Eq for State<C> {}

fn advance_bfs<C: Solvable>(
    visited: &mut HashSet<Rc<State<C>>>,
    queue: &mut VecDeque<Rc<State<C>>>,
) {
    let current_depth = queue.back().unwrap().past_moves.len();
    while let Some(rc_state) = queue.pop_front() {
        if rc_state.past_moves.len() > current_depth { return; }
        let past_moves = &rc_state.past_moves;
        let x = &rc_state.cube;
        for (m, y) in find_adjacents(x) {
            let new_moves = append_move(past_moves, m);
            let new_state = Rc::new(State {
                past_moves: new_moves.clone(),
                cube: y,
            });
            if !have_we_seen_this_state_before(visited, new_state.clone()) {
                visited.insert(new_state.clone());
                queue.push_back(new_state);
            }
        }
    }
}

fn have_we_seen_this_state_before<C: Solvable>(
    seen: &HashSet<Rc<State<C>>>,
    new: Rc<State<C>>,
) -> bool {
    seen.contains(&new) // Equality only depends on the cube
}

fn append_move(old: &[Move], m: Move) -> Vec<Move> {
    let mut new = old.to_owned();
    new.push(m);
    new
}

fn find_adjacents<C: Solvable>(x: &C) -> Vec<(Move, C)> {
    let moviments = C::moves_of_adjacency();

    let mut t = Vec::new();
    for mov in moviments {
        let mut alt = x.clone();
        alt.make_move(&mov);
        t.push((mov, alt))
    }
    t
}

pub trait Solvable: Display + Eq + Sized + Default + Clone + Hash {
    fn moves_of_adjacency() -> Vec<Move>;
    fn make_move(&mut self, movimement: &Move);
    const INPUT_FILE_NAME: &'static str;
    fn write_blank_slate() -> Result<(), Box<dyn Error>>;
    fn read_from_slate() -> Result<Self, Box<dyn Error>>;

    fn solve_random(scramble_length: usize) {
        println!("[INFO]: Generating random cube (n={scramble_length})...");
        let scrambling_instant = Instant::now();
        let (mut cube, scramble) = Self::random_scramble(scramble_length);
        let time_taken_to_scramble = scrambling_instant.elapsed();
        println!(
            "[INFO]: Scrambling took: {}ms ({}μs)",
            time_taken_to_scramble.as_millis(),
            time_taken_to_scramble.as_micros()
        );
        print!("[INFO]: Scramble is: ");
        println!("{scramble}");
        print!("[INFO]: (Uncompressed: [ ");
        for m in &scramble.0 {
            print!("{m} ");
        }
        println!("])");

        println!("[INFO]: Solving...");
        println!("Scramble to solve:\n{cube}");

        let starting_instant = Instant::now();
        let r = cube.solve();
        let time_taken = starting_instant.elapsed();

        for m in &r.0 {
            cube.make_move(m)
        }
        println!("Final state:\n{cube}");

        println!();

        println!(
            "[RESULT]: Solving time was: {}ms ({}μs)",
            time_taken.as_millis(),
            time_taken.as_micros()
        );
        println!("[RESULT]: Final solution is: {r}");
        print!("[INFO]: Uncompressed solution: [ ");
        for m in &r.0 {
            print!("{m} ");
        }
        println!("]");

        println!();

        println!("[RESULT]: Reverse of solution: {}", r.reversed());
        print!("[INFO]: Uncompressed reverse: [ ");
        for m in r.0.iter().rev() {
            print!("{} ", m.opposite());
        }
        println!("]");
    }
    /// Calls other Solvable methods with interspersed prints
    fn solve_pretty() {
        println!("[INFO]: Reading from `{}`...", Self::INPUT_FILE_NAME);
        let cube: Self = match Self::read_from_slate() {
            Ok(c) => c,
            Err(e) => {
                println!(
                    "[ERROR]: Could not parse `{0}`:'{e}'. Please double check `{0}`",
                    Self::INPUT_FILE_NAME
                );
                std::process::exit(2);
            }
        };
        println!("[INFO]: `{}` has been read", Self::INPUT_FILE_NAME);
        println!("[INFO]: Interpreted cube is:\n{cube}");
        println!("[INFO]: Starting the solve...");
        let r = cube.solve();

        println!("[INFO]: Checking correctness...");
        let mut checking_cube = cube.clone();
        for m in &r.0 {
            checking_cube.make_move(m)
        }

        println!("Starting cube:\n{cube}\n");
        println!("Final cube:\n{checking_cube}");

        println!("[RESULT]: Final solution is: {r}");
        print!("[INFO]: Uncompressed solution: [ ");
        for m in &r.0 {
            print!("{m} ");
        }
        println!("]");

        println!();

        println!("[RESULT]: Reverse of solution: {}", r.reversed());
        print!("[INFO]: Uncompressed reverse: [ ");
        for m in r.0.iter().rev() {
            print!("{} ", m.opposite());
        }
        println!("]");
    }

    fn solve(&self) -> MoveSeq {
        let first_state_unsolved = Rc::new(State {
            past_moves: Vec::new(),
            cube: self.clone(),
        });
        let mut w_from_unsolved = HashSet::from([first_state_unsolved.clone()]);
        let mut queue_from_unsolved = VecDeque::from([first_state_unsolved]);

        let first_state_solved = Rc::new(State {
            past_moves: Vec::new(),
            cube: Self::default(),
        });
        let mut w_from_solved = HashSet::from([first_state_solved.clone()]);
        let mut queue_from_solved = VecDeque::from([first_state_solved]);

        while w_from_solved.is_disjoint(&w_from_unsolved) {
            advance_bfs(&mut w_from_unsolved, &mut queue_from_unsolved);
            advance_bfs(&mut w_from_solved, &mut queue_from_solved);
        }
        println!();

        println!("[INFO]: Found solution after exploring: {} states from unsolved and {} states from solved",
            w_from_unsolved.len(),
            w_from_solved.len(),
        );

        // TODO: This only prints one solution, even though we've likely found many. This should be iterated through and the "best" one picked out.
        //       The definition of "best" should include something like length and the ratio of 'nice' moves (U, F, R) to 'weird' moves (the rest, like B')
        println!(
            "[INFO]: Number of intersecting states found is: {}",
            w_from_solved.intersection(&w_from_unsolved).count()
        );
        let schrodinger_state: &State<_> =
            (*w_from_solved.intersection(&w_from_unsolved).next().unwrap()).deref();
        let mut path_from_unsolved: Vec<Move> = w_from_unsolved
            .get(schrodinger_state).unwrap()
            .past_moves
            .clone();
        let path_from_solved: Vec<Move> = w_from_solved
            .get(schrodinger_state).unwrap()
            .past_moves
            .clone();

        println!("[INFO]: Found halves of the math: merging...");

        for m in path_from_solved.into_iter().rev() {
            path_from_unsolved.push(m.opposite());
        }

        println!("[INFO]: Verifying solution...");
        let mut test_cube = self.clone();
        for m in &path_from_unsolved {
            test_cube.make_move(m);
        }
        if test_cube == Self::default() { println!("[INFO]: Verification succeeded") }
        else { println!("[ERROR]: Verification incorrect, missing moves for linking rotation") }

        path_from_unsolved.into()
    }
    fn cycle_elements<const N: usize>(
        pieces: &mut [Piece; N],
        mut seq: [usize; 4],
        mov @ Move { side: _, prime }: &Move,
    ) {
        if *prime { seq = reverse_seq(seq); }
        cycle_items(pieces, seq); // Move the pieces
        for i in seq { pieces[i].rotate(mov) } // Rotate the pieces we just cycled around
    }

    fn scramble(moves: &MoveSeq) -> Self {
        let mut c = Self::default();
        for m in moves.iter() { c.make_move(m) }
        c
    }
    fn random_scramble(length: usize) -> (Self, MoveSeq) {
        fn get_move_from_n(n: usize) -> Move {
            match n % 12 {
                0 => Move::new("R"),
                1 => Move::new("L"),
                2 => Move::new("R"),
                3 => Move::new("B"),
                4 => Move::new("U"),
                5 => Move::new("D"),
                6 => Move::new("R'"),
                7 => Move::new("L'"),
                8 => Move::new("R'"),
                9 => Move::new("B'"),
                10 => Move::new("U'"),
                11 => Move::new("D'"),
                _ => unreachable!("Range reaches 12"),
            }
        }

        let mut scramble = vec![];
        for _ in 0..length {
            scramble.push(get_move_from_n(random_number_in_range(12)))
        }

        let c = Self::scramble(&scramble.clone().into());
        (c, scramble.into())
    }
}

pub const SIDE_RIGHT: usize = 0;
pub const SIDE_FRONT: usize = 1;
pub const SIDE_TOP: usize = 2;
pub const SIDE_LEFT: usize = 3;
pub const SIDE_BACK: usize = 4;
pub const SIDE_DOWN: usize = 5;

pub const COLOR_RIGHT_SEQ: [usize; 4] = [SIDE_FRONT, SIDE_TOP, SIDE_BACK, SIDE_DOWN];
pub const COLOR_LEFT_SEQ: [usize; 4] = [SIDE_DOWN, SIDE_BACK, SIDE_TOP, SIDE_FRONT];
pub const COLOR_UP_SEQ: [usize; 4] = [SIDE_FRONT, SIDE_LEFT, SIDE_BACK, SIDE_RIGHT];
pub const COLOR_DOWN_SEQ: [usize; 4] = [SIDE_RIGHT, SIDE_BACK, SIDE_LEFT, SIDE_FRONT];
pub const COLOR_FRONT_SEQ: [usize; 4] = [SIDE_TOP, SIDE_RIGHT, SIDE_DOWN, SIDE_LEFT];
pub const COLOR_BACK_SEQ: [usize; 4] = [SIDE_LEFT, SIDE_DOWN, SIDE_RIGHT, SIDE_TOP];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub side: MoveSide,
    pub prime: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveSide {
    R,
    F,
    U,
    L,
    B,
    D,
}

impl Move {
    pub fn new(s: &str) -> Move {
        if s.len() > 2 {
            panic!("{s} no és un moviment legal");
        }
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

        Move {
            side: m,
            prime: k.is_some(),
        }
    }
    pub fn opposite(&self) -> Self {
        Self {
            prime: !self.prime,
            ..*self
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoveSeq(pub Vec<Move>);

impl MoveSeq {
    pub fn reversed(&self) -> Self {
        Self(self.0.iter().rev().map(|m| m.opposite()).collect())
    }
}

impl From<Vec<Move>> for MoveSeq {
    fn from(value: Vec<Move>) -> Self {
        Self(value)
    }
}

impl Deref for MoveSeq {
    type Target = Vec<Move>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for MoveSeq {
    type Item = Move;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExpandedMove {
    L { prime: bool }, L2,
    R { prime: bool }, R2,
    F { prime: bool }, F2,
    B { prime: bool }, B2,
    U { prime: bool }, U2,
    D { prime: bool }, D2,
    Nothing,
}
impl ExpandedMove {
    /// This preserves rotations: so R L *could* be rewritten as 2L or 2R while preserving equality but, since it does not preserve rotation, it will not be compressed
    fn compress(a: Self, b: Self) -> Option<Self> {
        match (a, b) {
            (Self::Nothing, _) | (_, Self::Nothing) => None,
            (Self::L { prime: p1 }, Self::L { prime: p2 })
            | (Self::R { prime: p1 }, Self::R { prime: p2 })
            | (Self::F { prime: p1 }, Self::F { prime: p2 })
            | (Self::B { prime: p1 }, Self::B { prime: p2 })
            | (Self::B { prime: p1 }, Self::B { prime: p2 })
                if p1 != p2 =>
            {
                Some(Self::Nothing)
            }
            (Self::L2, Self::L2)
            | (Self::R2, Self::R2)
            | (Self::F2, Self::D2)
            | (Self::B2, Self::B2)
            | (Self::U2, Self::U2)
            | (Self::D2, Self::D2) => Some(Self::Nothing),
            (Self::L { prime: p1 }, Self::L { prime: p2 }) if p1 == p2 => Some(Self::L2),
            (Self::R { prime: p1 }, Self::R { prime: p2 }) if p1 == p2 => Some(Self::R2),
            (Self::F { prime: p1 }, Self::F { prime: p2 }) if p1 == p2 => Some(Self::F2),
            (Self::B { prime: p1 }, Self::B { prime: p2 }) if p1 == p2 => Some(Self::B2),
            (Self::U { prime: p1 }, Self::U { prime: p2 }) if p1 == p2 => Some(Self::U2),
            (Self::D { prime: p1 }, Self::D { prime: p2 }) if p1 == p2 => Some(Self::D2),
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
            _ => None,
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
                Move {
                    side: MoveSide::L,
                    prime,
                } => ExpandedMove::L { prime: *prime },
                Move {
                    side: MoveSide::R,
                    prime,
                } => ExpandedMove::R { prime: *prime },
                Move {
                    side: MoveSide::F,
                    prime,
                } => ExpandedMove::F { prime: *prime },
                Move {
                    side: MoveSide::B,
                    prime,
                } => ExpandedMove::B { prime: *prime },
                Move {
                    side: MoveSide::U,
                    prime,
                } => ExpandedMove::U { prime: *prime },
                Move {
                    side: MoveSide::D,
                    prime,
                } => ExpandedMove::D { prime: *prime },
            };
            stack.push(ext);
            while stack.len() > 1 {
                if let Some(c) =
                    ExpandedMove::compress(stack[stack.len() - 2], stack[stack.len() - 1])
                {
                    stack.pop();
                    stack.pop();
                    stack.push(c);
                } else {
                    break;
                }
            }
        }
        let mut o = String::new();
        for m in stack {
            if m != ExpandedMove::Nothing {
                o.push_str(&format!("{} ", m))
            };
        }

        write!(f, "[ {o} ]")
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
        if self.prime {
            out.push('\'')
        }

        write!(f, "{out}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Piece {
    pub rotation: PieceRotation,
}

/// Stored as [top color][front color], which uniquely defines a rotation (because the cross product isn't commutative!)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PieceRotation {
    WR, WO, WG, WB,
    RW, BW, OW, GW,
    RY, BY, OY, GY,
    OG, GO, OB, BO,
    RG, GR, RB, BR,
    YR, YB, YG,
    #[default] YO,
}

pub fn cycle_items<T: Clone, const N: usize>(v: &mut [T; N], idxs: [usize; 4]) {
    v.swap(idxs[0], idxs[1]);
    v.swap(idxs[0], idxs[2]);
    v.swap(idxs[0], idxs[3]);
}

pub fn reverse_seq([a, b, c, d]: [usize; 4]) -> [usize; 4] {
    [d, c, b, a]
}


pub fn random_number_in_range(max: usize) -> usize {
    let nanos: usize = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap().subsec_nanos()
        .try_into().unwrap();

    nanos % max
}
