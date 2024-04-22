pub mod declarations;
pub use declarations::*;

pub mod bfsing;
pub use bfsing::*;

pub mod input;
pub use input::*;

pub mod colors;
pub use colors::*;


// Position constants
const P_TOP_RIGHT_FRONT: usize    = 0;
const P_TOP_RIGHT_BACK: usize     = 1;
const P_TOP_LEFT_BACK: usize      = 2;
const P_TOP_LEFT_FRONT: usize     = 3;
const P_BOTTOM_RIGHT_FRONT: usize = 4;
const P_BOTTOM_RIGHT_BACK: usize  = 5;
const P_BOTTOM_LEFT_BACK: usize   = 6;
const P_BOTTOM_LEFT_FRONT: usize  = 7;

const SIDE_RIGHT: usize = 0;
const SIDE_FRONT: usize = 1;
const SIDE_TOP: usize   = 2;
const SIDE_LEFT: usize  = 3;
const SIDE_BACK: usize  = 4;
const SIDE_DOWN: usize  = 5;
 

// Rotation constants
const COLOR_RIGHT_SEQ: [usize; 4] = [SIDE_FRONT, SIDE_TOP, SIDE_BACK, SIDE_DOWN];
const COLOR_LEFT_SEQ: [usize; 4]  = [SIDE_DOWN, SIDE_BACK, SIDE_TOP, SIDE_FRONT];
const COLOR_UP_SEQ: [usize; 4]    = [SIDE_FRONT, SIDE_LEFT, SIDE_BACK, SIDE_RIGHT];
const COLOR_DOWN_SEQ: [usize; 4]  = [SIDE_RIGHT, SIDE_BACK, SIDE_LEFT, SIDE_FRONT];
const COLOR_FRONT_SEQ: [usize; 4] = [SIDE_TOP, SIDE_RIGHT, SIDE_DOWN, SIDE_LEFT];
const COLOR_BACK_SEQ: [usize; 4]  = [SIDE_LEFT, SIDE_DOWN, SIDE_RIGHT, SIDE_TOP];

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


pub fn print_solution(moves: &Vec<Move>) {
    print!("Solution is:  [ ");
    for m in moves {
    	print!("{m} ");
    }
    println!("]")
}

pub fn print_reverse_solution(moves: &Vec<Move>) {
    print!("Reverse of solution: [ ");
    for m in moves.into_iter().rev() {
    	print!("{} ", m.opposite());
    }
    println!("]")
}
