pub mod declarations;
pub use declarations::*;

pub mod bfsing;
pub use bfsing::*;

pub mod input;
pub use input::*;

pub use shared::*;

#[cfg(test)]
mod tests;

// Position constants
const P_TOP_RIGHT_FRONT: usize    = 0;
const P_TOP_RIGHT_BACK: usize     = 1;
const P_TOP_LEFT_BACK: usize      = 2;
const P_TOP_LEFT_FRONT: usize     = 3;
const P_BOTTOM_RIGHT_FRONT: usize = 4;
const P_BOTTOM_RIGHT_BACK: usize  = 5;
const P_BOTTOM_LEFT_BACK: usize   = 6;
const P_BOTTOM_LEFT_FRONT: usize  = 7;
 

// Rotation constants (could've been an enum, but eh)
const FACE_RIGHT_SEQ_CYCLE: [usize; 4] = [P_TOP_RIGHT_BACK, P_BOTTOM_RIGHT_BACK, P_BOTTOM_RIGHT_FRONT, P_TOP_RIGHT_FRONT];
const FACE_LEFT_SEQ_CYCLE: [usize; 4]  = [P_TOP_LEFT_FRONT, P_BOTTOM_LEFT_FRONT, P_BOTTOM_LEFT_BACK, P_TOP_LEFT_BACK];
const FACE_UP_SEQ_CYCLE: [usize; 4]    = [P_TOP_LEFT_FRONT, P_TOP_LEFT_BACK, P_TOP_RIGHT_BACK, P_TOP_RIGHT_FRONT];
const FACE_DOWN_SEQ_CYCLE: [usize; 4]  = [P_BOTTOM_LEFT_BACK, P_BOTTOM_LEFT_FRONT, P_BOTTOM_RIGHT_FRONT, P_BOTTOM_RIGHT_BACK];
const FACE_FRONT_SEQ_CYCLE: [usize; 4] = [P_BOTTOM_LEFT_FRONT, P_TOP_LEFT_FRONT, P_TOP_RIGHT_FRONT, P_BOTTOM_RIGHT_FRONT];
const FACE_BACK_SEQ_CYCLE: [usize; 4]  = [P_BOTTOM_LEFT_BACK, P_BOTTOM_RIGHT_BACK, P_TOP_RIGHT_BACK, P_TOP_LEFT_BACK];

 const FACE_RIGHT_SEQ_PRINT: [usize; 4] = [P_TOP_RIGHT_FRONT, P_TOP_RIGHT_BACK, P_BOTTOM_RIGHT_FRONT, P_BOTTOM_RIGHT_BACK];
 const FACE_LEFT_SEQ_PRINT: [usize; 4]  = [P_TOP_LEFT_BACK, P_TOP_LEFT_FRONT, P_BOTTOM_LEFT_BACK, P_BOTTOM_LEFT_FRONT];
 const FACE_UP_SEQ_PRINT: [usize; 4]    = [P_TOP_LEFT_BACK, P_TOP_RIGHT_BACK, P_TOP_LEFT_FRONT, P_TOP_RIGHT_FRONT];
 const FACE_DOWN_SEQ_PRINT: [usize; 4]  = [P_BOTTOM_LEFT_FRONT, P_BOTTOM_RIGHT_FRONT, P_BOTTOM_LEFT_BACK, P_BOTTOM_RIGHT_BACK];
 const FACE_FRONT_SEQ_PRINT: [usize; 4] = [P_TOP_LEFT_FRONT, P_TOP_RIGHT_FRONT, P_BOTTOM_LEFT_FRONT, P_BOTTOM_RIGHT_FRONT];
 const FACE_BACK_SEQ_PRINT: [usize; 4]  = [P_TOP_RIGHT_BACK, P_TOP_LEFT_BACK, P_BOTTOM_RIGHT_BACK, P_BOTTOM_LEFT_BACK];


impl Solvable for Cube2 {
    fn make_move(&mut self, m: Move) {
        match m.side() {
            MoveSide::R => Self::cycle_elements::<8>(&mut self.pieces, FACE_RIGHT_SEQ_CYCLE, m),
            MoveSide::L => Self::cycle_elements::<8>(&mut self.pieces, FACE_LEFT_SEQ_CYCLE , m),
            MoveSide::U => Self::cycle_elements::<8>(&mut self.pieces, FACE_UP_SEQ_CYCLE   , m),
            MoveSide::B => Self::cycle_elements::<8>(&mut self.pieces, FACE_BACK_SEQ_CYCLE , m),
            MoveSide::F => Self::cycle_elements::<8>(&mut self.pieces, FACE_FRONT_SEQ_CYCLE, m),
            MoveSide::D => Self::cycle_elements::<8>(&mut self.pieces, FACE_DOWN_SEQ_CYCLE , m),
        };
    }
    fn moves_of_adjacency() -> Vec<Move> {
        Vec::from([
            Move::R, Move::F, Move::U,
            Move::L, Move::B, Move::D
        ])
    }
}

fn get_orientation_generators() -> [Vec<Move>; 6] {
    [
	vec![],
	vec![Move::F, Move::B],
	vec![Move::R, Move::L],
	vec![Move::F, Move::B],
	vec![Move::R, Move::L],
	vec![Move::F, Move::B, Move::F, Move::B],
    ]
}
fn get_rotation_generators() -> [Vec<Move>; 4] {
    [
	vec![],
	vec![Move::U, Move::D],
	vec![Move::D, Move::U],
	vec![Move::U, Move::D, Move::U, Move::D],
    ]
}
