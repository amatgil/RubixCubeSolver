pub mod declarations;
pub use declarations::*;

pub mod bfsing;
pub use bfsing::*;

pub mod input;
pub use input::*;



// Rotation constants
const COLOR_RIGHT_SEQ: [usize; 4] = [1, 2, 4, 5];
const COLOR_LEFT_SEQ: [usize; 4]  = [5, 4, 2, 1];
const COLOR_UP_SEQ: [usize; 4]    = [0, 1, 3, 4];
const COLOR_DOWN_SEQ: [usize; 4]  = [4, 3, 1, 0];
const COLOR_FRONT_SEQ: [usize; 4] = [0, 5, 3, 2];
const COLOR_BACK_SEQ: [usize; 4]  = [2, 3, 5, 0];

const FACE_RIGHT_SEQ_CYCLE: [usize; 4] = [0, 1, 5, 4];
const FACE_LEFT_SEQ_CYCLE: [usize; 4]  = [3, 7, 6, 2];
const FACE_UP_SEQ_CYCLE: [usize; 4]    = [2, 1, 0, 3];
const FACE_DOWN_SEQ_CYCLE: [usize; 4]  = [7, 4, 5, 6];
const FACE_FRONT_SEQ_CYCLE: [usize; 4] = [3, 0, 4, 7];
const FACE_BACK_SEQ_CYCLE: [usize; 4]  = [1, 2, 6, 5];

const FACE_RIGHT_SEQ_PRINT: [usize; 4] = [0, 1, 4, 5];
const FACE_LEFT_SEQ_PRINT: [usize; 4]  = [2, 3, 6, 7];
const FACE_UP_SEQ_PRINT: [usize; 4]    = [2, 1, 3, 0];
const FACE_DOWN_SEQ_PRINT: [usize; 4]  = [5, 6, 4, 7];
const FACE_FRONT_SEQ_PRINT: [usize; 4] = [3, 0, 7, 4];
const FACE_BACK_SEQ_PRINT: [usize; 4]  = [1, 2, 5, 6];


fn cycle_items<T: Clone, const N: usize>(v: &mut [T; N], nums: [usize; 4]) {
    let e = v[nums[3]].clone();

    v[nums[3]] = v[nums[2]].clone();
    v[nums[2]] = v[nums[1]].clone();
    v[nums[1]] = v[nums[0]].clone();
    v[nums[0]] = e;
}
