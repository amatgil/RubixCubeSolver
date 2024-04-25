pub mod declarations;
pub use declarations::*;

pub mod bfsing;
pub use bfsing::*;

pub mod input;
pub use input::*;

pub mod drawing;
pub use drawing::*;

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
 

// Rotation constants

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

fn get_orientation_generators() -> [Vec<Move>; 6] {
    [
	vec![],
	vec![Move::new("F") , Move::new("B'")],
	vec![Move::new("R"), Move::new("L'")],
	vec![Move::new("F'"), Move::new("B")],
	vec![Move::new("R'"), Move::new("L'")],
	vec![Move::new("F"), Move::new("B"), Move::new("F"), Move::new("B")],
    ]
}
fn get_rotation_generators() -> [Vec<Move>; 4] {
    [
	vec![],
	vec![Move::new("U"), Move::new("D'")],
	vec![Move::new("D"), Move::new("U'")],
	vec![Move::new("U"), Move::new("D"), Move::new("U"), Move::new("D")],
    ]
}
