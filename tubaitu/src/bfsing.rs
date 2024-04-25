use crate::*;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::hash::Hash;
use std::io;
use std::io::Write;
use std::ops::Deref;
use std::rc::Rc;

pub fn is_solved(c: &Cube) -> bool {
    c.pieces.iter().fold((true, &c.pieces[0]), |(acc_b, acc_c), x| (acc_b && &acc_c == &x, x) ).0
}

#[test]
fn basic_is_solved_test() {
    let solved_piece = Piece { rotation: PieceRotation::WO };
    let mut cube: Cube = Cube { pieces: [solved_piece; 8] };

    assert!(is_solved(&cube));
    cube.make_move(&Move::new("R"));
    assert!(!is_solved(&cube));
}

#[derive(Debug, Clone)]
struct State {
    past_moves: Vec<Move>,
    cube: Cube,
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
	self.cube.hash(state);
    }
}

fn advance_bfs(visited: &mut HashSet<Rc<State>>, queue: &mut VecDeque<Rc<State>>) {
    let current_depth = queue.back().unwrap().past_moves.len();
    while let Some(rc_state) = queue.pop_front() {
	if rc_state.past_moves.len() > current_depth { return; }
	let past_moves = &rc_state.past_moves;
	let x = rc_state.cube;
        for (m, y) in find_adjacents(&x) {
	    let new_moves = append_move(&past_moves, m);
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

// TODO: Only check disjoint-ness between newly explored verticies
pub fn solve(cube: Cube) -> MoveSeq {
    let first_state_unsolved    = Rc::new(State { past_moves: Vec::new(), cube });
    let mut w_from_unsolved     = HashSet::from([first_state_unsolved.clone()]);
    let mut queue_from_unsolved = VecDeque::from([first_state_unsolved]);

    let first_state_solved    = Rc::new(State { past_moves: Vec::new(), cube: Cube::default() });
    let mut w_from_solved     = HashSet::from([first_state_solved.clone()]);
    let mut queue_from_solved = VecDeque::from([first_state_solved]);


    while w_from_solved.is_disjoint(&w_from_unsolved) {
	io::stdout().flush().unwrap();
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
    println!("[INFO]: Number of intersecting states found is: {}",
        w_from_solved.intersection(&w_from_unsolved).count()
    );
    let schrodinger_state: State = (*w_from_solved.intersection(&w_from_unsolved).next().unwrap()).deref().clone();
    let mut path_from_unsolved: Vec<Move> = w_from_unsolved.get(&schrodinger_state).unwrap().past_moves.clone();
    let path_from_solved: Vec<Move> = w_from_solved.get(&schrodinger_state).unwrap().past_moves.clone();

    println!("[INFO]: Found halves of the math: merging...");

    let mut reorient_a = cube.clone();
    let mut reorient_b = Cube::default();
    for m in &path_from_unsolved { reorient_a.make_move(m) }
    for m in &path_from_solved { reorient_b.make_move(m) }

    // Adjust for rotational symmetry. This seems to make it so the resulting solved cube is always WB, curiously
    let linking_moves = reorient_together(&reorient_a, &reorient_b).expect("This comes from two sets being non-disjoint, and so should never be reached.");
    for m in linking_moves { path_from_unsolved.push(m) }

    for m in path_from_solved.into_iter().rev() {
	path_from_unsolved.push(m.opposite());
    }

    path_from_unsolved.into()

}

/// Takes in two cubes. Returns a sequence of moves that will turn the left one into the right one
/// Not optimized for efficiency
fn reorient_together(a: &Cube, b: &Cube) -> Option<Vec<Move>> {
    for mut o in get_orientation_generators() {
	for mut r in get_rotation_generators() {
	    let mut alternate_cube = a.clone();
	    for m1 in &mut *o { alternate_cube.make_move(m1) }
	    for m2 in &r { alternate_cube.make_move(&m2) }
	    if alternate_cube.pieces == b.pieces {
		o.append(&mut r);
		return Some(o);
	    }
	}
    }
    None
}

fn append_move(old: &Vec<Move>, m: Move) -> Vec<Move> {
    let mut new = old.clone();
    new.push(m);
    new
}

impl std::cmp::PartialEq for State {
    fn eq(&self, rhs: &Self) -> bool {
       self.cube == rhs.cube
    }
}
impl Eq for State {}

fn find_adjacents(x: &Cube) -> Vec<(Move, Cube)>{
    let moviments: [Move; 6] = [
        Move::new("R"), Move::new("F"), Move::new("U"),
        Move::new("L'"), Move::new("B'"), Move::new("D'"),
    ];

    let mut t = Vec::new();
    for mov in moviments {
        let mut alt = x.clone();
        alt.make_move(&mov);
        t.push((mov, alt))
    }
    t
}

fn have_we_seen_this_state_before(seen: &HashSet<Rc<State>>, new: Rc<State>) -> bool {
    seen.contains(&new) // Equality only depends on the cube
}

#[test]
fn adjacent_test() {
    let solved_piece = Piece { rotation: PieceRotation::OW };
    let solved_cube: Cube = Cube { pieces: [solved_piece; 8] };
    let mut t = Vec::new();

    let moviments: [Move; 6] = [
        Move::new("R"), Move::new("F"), Move::new("U"),
        Move::new("L'"), Move::new("B'"), Move::new("D'"),
    ];
    for mov in moviments {
        let mut alt = solved_cube.clone();
        alt.make_move(&mov);
        t.push((mov, alt))
    }

    assert_eq!(t, find_adjacents(&solved_cube));
    
}


#[test]
fn only_right_solve() {
    let mut cube = Cube::scramble(&vec![Move::new("R")].into());
    for m in solve(cube) { cube.make_move(&m); }

    assert_eq!(cube, Cube::default());
}

#[test]
fn only_left_solve() {
    let mut cube = Cube::scramble(&vec![Move::new("L")].into());
    for m in solve(cube) { cube.make_move(&m); }

    assert_eq!(cube, Cube::default());
}

#[test]
fn double_up_solve() {
    let mut cube = Cube::scramble(&vec![
	Move::new("U"),
	Move::new("U"),
    ].into());
    for m in solve(cube) { cube.make_move(&m); }

    assert_eq!(cube, Cube::default());
}

#[test]
fn back_up_solve() {
    let mut cube = Cube::scramble(&vec![
	Move::new("B"),
	Move::new("U"),
    ].into());
    for m in solve(cube) { cube.make_move(&m); }

    assert_eq!(cube, Cube::default());
}

#[test]
fn redundant_solve() {
    let mut cube = Cube::scramble(&vec![
	Move::new("U"),
	Move::new("U"),
	Move::new("U"),
    ].into());
    for m in solve(cube) { cube.make_move(&m); }

    assert_eq!(cube, Cube::default());
}

#[test]
fn opposite_solve() {
    let mut cube = Cube::scramble(&vec![
	Move::new("L"),
	Move::new("R"),
    ].into());
    for m in solve(cube) { cube.make_move(&m); }

    assert_eq!(cube, Cube::default());
}

#[test]
fn complicated_solve() {
    let mut cube = Cube::scramble(&vec![
	Move::new("R"),
	Move::new("U"),
	Move::new("L"),
	Move::new("D"),
	Move::new("F"),
    ].into());
    for m in solve(cube) { cube.make_move(&m); }

    assert_eq!(cube, Cube::default());
}

#[test]
fn reorientation() {
    let a = Cube::default();
    let mut b = Cube::default();
    b.make_move(&Move::new("R"));
    b.make_move(&Move::new("L'"));

    let answer = vec![Move::new("R"), Move::new("L'")];
    assert_eq!(answer, reorient_together(&a, &b).unwrap());
}

#[test]
fn reorientation2() {
    let a = Cube::default();
    let mut b = Cube::default();
    b.make_move(&Move::new("U"));
    b.make_move(&Move::new("U"));
    b.make_move(&Move::new("D'"));
    b.make_move(&Move::new("D'"));

    let answer = vec![Move::new("U"), Move::new("D"), Move::new("U"), Move::new("D")];
    assert_eq!(answer, reorient_together(&a, &b).unwrap());
}
