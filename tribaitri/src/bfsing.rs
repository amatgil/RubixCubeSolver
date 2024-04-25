use shared::MoveSeq;
use std::hash::Hash;

use crate::*;

#[derive(Debug, Clone)]
struct State {
    past_moves: Vec<Move>,
    cube: Cube3,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool { self.cube == other.cube }
}

impl Eq for State {}
impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
	self.cube.hash(state);
    }
}

pub(crate) fn solve_three_by_three(cube: &Cube3) -> MoveSeq {
    todo!()
}

fn advance_bfs(visited: &mut HashSet<Rc<State>>, queue: &mut VecDeque<Rc<State>>) {
    let current_depth = queue.back().unwrap().past_moves.len();
    while let Some(rc_state) = queue.pop_front() {
	if rc_state.past_moves.len() > current_depth { return; }
	let past_moves = &rc_state.past_moves;
	let x = &rc_state.cube;
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

fn have_we_seen_this_state_before(seen: &HashSet<Rc<State>>, new: Rc<State>) -> bool {
    seen.contains(&new) // Equality only depends on the cube
}

fn append_move(old: &Vec<Move>, m: Move) -> Vec<Move> {
    let mut new = old.clone();
    new.push(m);
    new
}

fn find_adjacents(x: &Cube3) -> Vec<(Move, Cube3)>{
    let moviments: [Move; 12] = [
        Move::new("R"), Move::new("R'"),
        Move::new("L"), Move::new("L'"),
        Move::new("F"), Move::new("F'"),
        Move::new("B"), Move::new("B'"),
        Move::new("U"), Move::new("U'"),
        Move::new("D"), Move::new("D'"),
    ];

    let mut t = Vec::new();
    for mov in moviments {
        let mut alt = x.clone();
        alt.make_move(&mov);
        t.push((mov, alt))
    }
    t
}
