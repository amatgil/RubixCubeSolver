pub mod scene;
pub use scene::*;

pub mod cube2x2;
pub use cube2x2::*;

pub mod piece;
pub use piece::*;

pub mod stiker;
pub use stiker::*;


pub mod utils;
pub use utils::*;

pub use shared::*;


const DEFAULT_CAMERA_PLANE_DISTANCE: f64 = 1.0;

const P_TOP_RIGHT_FRONT: usize    = 0;
const P_TOP_RIGHT_BACK: usize     = 1;
const P_TOP_LEFT_BACK: usize      = 2;
const P_TOP_LEFT_FRONT: usize     = 3;
const P_BOTTOM_RIGHT_FRONT: usize = 4;
const P_BOTTOM_RIGHT_BACK: usize  = 5;
const P_BOTTOM_LEFT_BACK: usize   = 6;
const P_BOTTOM_LEFT_FRONT: usize  = 7;

const FACE_RIGHT_SEQ_CYCLE: [usize; 4] = [P_TOP_RIGHT_BACK, P_BOTTOM_RIGHT_BACK, P_BOTTOM_RIGHT_FRONT, P_TOP_RIGHT_FRONT];
const FACE_LEFT_SEQ_CYCLE: [usize; 4]  = [P_TOP_LEFT_FRONT, P_BOTTOM_LEFT_FRONT, P_BOTTOM_LEFT_BACK, P_TOP_LEFT_BACK];
const FACE_UP_SEQ_CYCLE: [usize; 4]    = [P_TOP_LEFT_FRONT, P_TOP_LEFT_BACK, P_TOP_RIGHT_BACK, P_TOP_RIGHT_FRONT];
const FACE_DOWN_SEQ_CYCLE: [usize; 4]  = [P_BOTTOM_LEFT_BACK, P_BOTTOM_LEFT_FRONT, P_BOTTOM_RIGHT_FRONT, P_BOTTOM_RIGHT_BACK];
const FACE_FRONT_SEQ_CYCLE: [usize; 4] = [P_BOTTOM_LEFT_FRONT, P_TOP_LEFT_FRONT, P_TOP_RIGHT_FRONT, P_BOTTOM_RIGHT_FRONT];
const FACE_BACK_SEQ_CYCLE: [usize; 4]  = [P_BOTTOM_LEFT_BACK, P_BOTTOM_RIGHT_BACK, P_TOP_RIGHT_BACK, P_TOP_LEFT_BACK];

fn get_corner_cycle(mov: Move) -> [usize; 4] {
    match mov {
        Move::R => FACE_RIGHT_SEQ_CYCLE,
        Move::L => FACE_LEFT_SEQ_CYCLE,
        Move::U => FACE_UP_SEQ_CYCLE,
        Move::D => FACE_DOWN_SEQ_CYCLE,
        Move::F => FACE_FRONT_SEQ_CYCLE,
        Move::B => FACE_BACK_SEQ_CYCLE,
        _ => [0; 4],
    }
}