use std::{collections::{HashSet, VecDeque}, fmt::Display, rc::Rc};

use shared::*;

mod bfsing;
use bfsing::*;

#[derive(Debug, Clone, Hash)]
pub struct Cube3 {
    left:  [Rc<Piece>; 8], // Orange
    right: [Rc<Piece>; 8], // Orange
    front: [Rc<Piece>; 8], // Orange
    back:  [Rc<Piece>; 8], // Orange
    top:   [Rc<Piece>; 8], // Orange
    down:  [Rc<Piece>; 8], // Orange
}


impl Default for Cube3 {
    fn default() -> Self {
        todo!()
    }
}
    
impl Solvable for Cube3 {
    fn solve(&self) -> MoveSeq {
        solve_three_by_three(self)
    }
    fn make_move(&mut self, m: &Move) {
        todo!()
    }
    fn random_scramble(length: usize) -> (Self, MoveSeq) {
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
        for _ in 0..length { scramble.push(get_move_from_n(rand::thread_rng().gen_range(0..12))) }

        let c = Cube3::scramble(&scramble.clone().into());
	(c, scramble.into())
    }

    fn scramble(moves: &MoveSeq) -> Self {
	let mut c = Cube3::default();
	for m in moves.iter() { c.make_move(&m) }
        c
    }
}


impl Display for Cube3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl PartialEq for Cube3 {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
impl Eq for Cube3 {}
