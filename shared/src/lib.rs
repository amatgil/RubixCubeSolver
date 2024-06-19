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

#[derive(Clone)]
struct State<C> {
    past_state: Option<(Rc<State<C>>, Move)>, 
    length_of_path: usize,
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
    let current_depth = queue.back().unwrap().length_of_path;
    while let Some(state) = queue.pop_front() {
        if state.length_of_path > current_depth { return; }
        for (m, y) in find_adjacents(&state.cube) {
            let new_state = Rc::new(State {
                past_state: Some((Rc::clone(&state), m)),
                length_of_path: state.length_of_path + 1,
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

fn find_adjacents<C: Solvable>(x: &C) -> Vec<(Move, C)> {
    let moviments = C::moves_of_adjacency();

    let mut t = Vec::new();
    for mov in moviments {
        let mut alt = x.clone();
        alt.make_move(mov);
        t.push((mov, alt))
    }
    t
}

pub trait Solvable: Display + Eq + Sized + Default + Clone + Hash {
    fn moves_of_adjacency() -> Vec<Move>;
    fn make_move(&mut self, movimement: Move);
    const INPUT_FILE_NAME: &'static str;
    fn write_blank_slate() -> Result<(), Box<dyn Error>>;
    fn read_from_slate() -> Result<Self, Box<dyn Error>>;

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
                std::process::exit(2)
            }
        };
        println!("[INFO]: `{}` has been read", Self::INPUT_FILE_NAME);
        println!("[INFO]: Interpreted cube is:\n{cube}");
        println!("[INFO]: Starting the solve...");
        let r = cube.solve(true);

        println!("[INFO]: Checking correctness...");
        let mut checking_cube = cube.clone();
        for m in &r.0 {
            checking_cube.make_move(*m)
        }

        println!("Starting cube:\n{cube}\n");
        println!("Final cube:\n{checking_cube}");

        println!("[RESULT]: Final solution is: {}", r);
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

    fn solve(&self, prints_enabled: bool) -> MoveSeq {
        let first_state_unsolved = Rc::new(State {
            cube: self.clone(),
            past_state: None,
            length_of_path: 0,
        });
        let mut w_from_unsolved: HashSet<Rc<State<Self>>> = HashSet::from([first_state_unsolved.clone()]);
        let mut queue_from_unsolved: VecDeque<Rc<State<Self>>> = VecDeque::from([first_state_unsolved]);

        let first_state_solved = Rc::new(State {
            past_state: None,
            length_of_path: 0,
            cube: Self::default(),
        });
        let mut w_from_solved = HashSet::from([first_state_solved.clone()]);
        let mut queue_from_solved = VecDeque::from([first_state_solved]);

        while w_from_solved.is_disjoint(&w_from_unsolved) {
            advance_bfs(&mut w_from_unsolved, &mut queue_from_unsolved);
            advance_bfs(&mut w_from_solved, &mut queue_from_solved);
            print!("\rEstats visitats: {} i {}", w_from_unsolved.len(), w_from_solved.len());
            use std::io::Write;
            std::io::stdout().flush().unwrap()
        }
        if prints_enabled {
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
        }
        
        let schrodinger_state: &State<_> =
            (*w_from_solved.intersection(&w_from_unsolved).next().unwrap()).deref();
        let mut path_from_unsolved: Vec<Move> =
            extract_path_from_first_state(w_from_unsolved.get(schrodinger_state).cloned());
        let path_from_solved: Vec<Move> =
            extract_path_from_first_state(w_from_solved.get(schrodinger_state).cloned());

        if prints_enabled {
            println!("[INFO]: Found halves of the math: merging...");

            for m in path_from_solved.clone().into_iter().rev() {
                path_from_unsolved.push(m.opposite());
            }
            println!("[INFO]: Verifying solution...");
        }

        let mut test_cube = self.clone();
        for m in &path_from_unsolved {
            test_cube.make_move(*m);
        }
        if prints_enabled {
            if test_cube == Self::default() { println!("[INFO]: Verification succeeded") }
            else { println!("[ERROR]: Verification incorrect, missing moves for linking rotation") }
        }

        path_from_unsolved.into()
    }

    fn cycle_elements<const N: usize>(
        pieces: &mut [Piece; N],
        mut seq: [usize; 4],
        mov: Move,
    ) {
        if mov.is_prime() { seq = reverse_seq(seq); }
        cycle_items(pieces, seq); // Move the pieces
        for i in seq { pieces[i].rotate(mov) } // Rotate the pieces we just cycled around
    }

    fn scramble(moves: &MoveSeq) -> Self {
        let mut c = Self::default();
        for m in moves.iter() { c.make_move(*m) }
        c
    }
    fn random_scramble(length: usize) -> (Self, MoveSeq) {
        fn get_move_from_n(n: usize) -> Move { Move(n as u8) }

        let mut scramble = vec![];
        for _ in 0..length {
            scramble.push(get_move_from_n(random_number_in_range(12)))
        }

        let c = Self::scramble(&scramble.clone().into());
        (c, scramble.into())
    }
}

/// inefficient, but only called twice so eh
fn extract_path_from_first_state<C>(s: Option<Rc<State<C>>>) -> Vec<Move> {
    let mut v = VecDeque::new();
    if s.is_none() { return v.into(); }
    let mut s: Rc<State<C>> = s.unwrap();
    while let Some((past_state, m)) = &s.past_state {
        v.push_front(*m);
        if past_state.past_state.is_none() { break; }
        s = (s.past_state).clone().unwrap().0;
    }

    v.into()
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

/// A move, internally represented by a single u8 using bit magic
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move(u8);

#[repr(u8)]
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
    pub const R:  Move = Self(0);
    pub const RP: Move = Self(1);
    pub const F:  Move = Self(2);
    pub const FP: Move = Self(3);
    pub const U:  Move = Self(4);
    pub const UP: Move = Self(5);
    pub const L:  Move = Self(6);
    pub const LP: Move = Self(7);
    pub const B:  Move = Self(8);
    pub const BP: Move = Self(9);
    pub const D:  Move = Self(10);
    pub const DP: Move = Self(11);

    pub fn opposite(&self) -> Self {
        Self(self.0 ^ 1)
    }
    pub fn is_prime(&self) -> bool {
        (self.0 & 1) != 0
    }
    pub fn side(&self) -> MoveSide {
        match *self {
            Self::R | Self::RP => MoveSide::R,
            Self::F | Self::FP => MoveSide::F,
            Self::U | Self::UP => MoveSide::U,
            Self::L | Self::LP => MoveSide::L,
            Self::B | Self::BP => MoveSide::B,
            Self::D | Self::DP => MoveSide::D,
            _ => unreachable!(), // TODO: Use _unchecked when tests pass
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
            let ext = match mov.side() {
                MoveSide::L => ExpandedMove::L { prime: mov.is_prime() },
                MoveSide::R => ExpandedMove::R { prime: mov.is_prime() },
                MoveSide::F => ExpandedMove::F { prime: mov.is_prime() },
                MoveSide::B => ExpandedMove::B { prime: mov.is_prime() },
                MoveSide::U => ExpandedMove::U { prime: mov.is_prime() },
                MoveSide::D => ExpandedMove::D { prime: mov.is_prime() },
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

        write!(f, "{o}")
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut out = String::new();
        out.push(match self.side() {
            MoveSide::R => 'R',
            MoveSide::F => 'F',
            MoveSide::U => 'U',
            MoveSide::L => 'L',
            MoveSide::B => 'B',
            MoveSide::D => 'D',
        });
        if self.is_prime() {
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

#[test]
fn opposite_moves() {
    assert_eq!(Move::R.opposite(), Move::RP);
    assert_eq!(Move::RP.opposite(), Move::R);
    assert_eq!(Move::L.opposite(), Move::LP);
    assert_eq!(Move::LP.opposite(), Move::L);
    assert_eq!(Move::F.opposite(), Move::FP);
    assert_eq!(Move::FP.opposite(), Move::F);
    assert_eq!(Move::U.opposite(), Move::UP);
    assert_eq!(Move::UP.opposite(), Move::U);
}

#[test]
fn primeness() {
    assert!(!Move::R.is_prime());
    assert!(!Move::L.is_prime());
    assert!(!Move::F.is_prime());
    assert!(!Move::B.is_prime());
    assert!(!Move::U.is_prime());
    assert!(!Move::D.is_prime());
    assert!( Move::RP.is_prime());
    assert!( Move::LP.is_prime());
    assert!( Move::FP.is_prime());
    assert!( Move::BP.is_prime());
    assert!( Move::UP.is_prime());
    assert!( Move::DP.is_prime());
}
