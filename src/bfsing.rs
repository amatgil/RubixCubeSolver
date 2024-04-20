use crate::*;
use std::collections::VecDeque;
use std::collections::HashSet;

pub fn is_solved(c: &Cube) -> bool {
    //c.pieces.iter().fold((true, &c.pieces[0]), |(acc_b, acc_c), x| (acc_b && &acc_c == &x, x) ).0
    let p = &c.pieces;
    p[0] == p[1] && p[1] == p[2] && p[2] == p[3] && p[3] == p[4] && p[4] == p[5] && p[5] == p[6] && p[6] == p[7]
}

#[test]
fn basic_is_solved_test() {
    let solved_piece = Piece::new(['G', 'O', 'W', 'B', 'R', 'Y']);
    let mut cube: Cube = Cube { pieces: [solved_piece; 8] };

    assert!(is_solved(&cube));
    cube.make_move(&Move::new("R"));
    assert!(!is_solved(&cube));
}

#[derive(Hash, Debug, Clone)]
struct State {
    past_moves: Vec<Move>,
    cube: Cube,
}

pub fn solve(cube: Cube) -> Vec<Move> {
    let first_state = State { past_moves: Vec::new(), cube, };
    let mut w: HashSet<State> = HashSet::from([first_state.clone()]);
    let mut queue: VecDeque<State> = VecDeque::from([first_state]);

    let mut i = 0;
    while let Some(State { mut past_moves, cube: x }) = queue.pop_front() {
        if i % 1 == 0 { println!("Iteration is: {i}"); }
        i += 1;

        if is_solved(&x) { return past_moves; }
        for (m, y) in find_adjacents(&x) {
            past_moves.push(m);
            let new_state = State {
                past_moves: past_moves.clone(),
                cube: y,
            };
            if !have_we_seen_this_state_before(&w, &new_state) {
                w.insert(new_state.clone());
                queue.push_back(new_state);
            }
        }
    }

    panic!("Entire tree was explored, no solved state was found. A corner must've been twisted or smth");
}

impl std::cmp::PartialEq for State {
    fn eq(&self, rhs: &Self) -> bool {
       self.cube == rhs.cube
    }
}
impl Eq for State {}

fn find_adjacents(x: &Cube) -> Vec<(Move, Cube)>{
    let moviments: [Move; 12] = [
        Move::new("R"), Move::new("R'"), Move::new("L"), Move::new("L'"), Move::new("U"), Move::new("U'"),
        Move::new("D"), Move::new("D'"), Move::new("F"), Move::new("F'"), Move::new("B"), Move::new("B'"),
    ];

    let mut t = Vec::new();
    for mov in moviments {
        let mut alt = x.clone();
        alt.make_move(&mov);
        t.push((mov, alt))
    }
    t
}

fn have_we_seen_this_state_before(seen: &HashSet<State>, new: &State) -> bool {
    seen.contains(&new) // Equality only depends on the cube
}

#[test]
fn adjacent_test() {
    let solved_piece = Piece::new(['G', 'O', 'W', 'B', 'R', 'Y']);
    let solved_cube: Cube = Cube { pieces: [solved_piece; 8] };
    let mut t = Vec::new();

    let moviments: [Move; 12] = [
        Move::new("R"), Move::new("R'"), Move::new("L"), Move::new("L'"), Move::new("U"), Move::new("U'"),
        Move::new("D"), Move::new("D'"), Move::new("F"), Move::new("F'"), Move::new("B"), Move::new("B'"),
    ];
    for mov in moviments {
        let mut alt = solved_cube.clone();
        alt.make_move(&mov);
        t.push((mov, alt))
    }

    assert_eq!(t, find_adjacents(&solved_cube));
    
}