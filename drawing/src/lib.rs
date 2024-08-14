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