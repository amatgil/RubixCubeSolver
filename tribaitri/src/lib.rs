use std::{fmt::Display, rc::Rc};

use shared::*;

mod moving;
use moving::*;

/// A 3x3x3 Rubix's cube with the following orientation:
/// | Face  | Center Color |
/// |-------|--------------|
/// | Up    | Yellow       |
/// | Down  | White        |
/// | Left  | Green        |
/// | Right | Blue         |
/// | Front | Orange       |
/// | Back  | Red          |
///
/// Internally, it is represented by the 6 edges and 14 corners only. Their indicies are defined by the corresponding ThreeByCorner and ThreeByEdge enums.
/// Note that equality is direct here, unlike a 2x2: there are no symmetries to speak of because the centers ground us
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Cube3 {
    pieces: [Piece; 3*3*3 - 1 - 6], // 20: edges + corners
}


impl Default for Cube3 {
    fn default() -> Self {
        todo!()
    }
}
    
impl Solvable for Cube3 {
    fn moves_of_adjacency() -> Vec<Move> {
        Vec::from([
            Move::new("R"), Move::new("R'"),
            Move::new("L"), Move::new("L'"),
            Move::new("F"), Move::new("F'"),
            Move::new("B"), Move::new("B'"),
            Move::new("U"), Move::new("U'"),
            Move::new("D"), Move::new("D'")
        ])
    }
    fn make_move(&mut self, m: &Move) {
        make_three_by_three_move(self, m);
    }
}


impl Display for Cube3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
