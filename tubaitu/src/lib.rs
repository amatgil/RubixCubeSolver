pub mod declarations;
use std::{error::Error, fs::{self, File}, io::Write};

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

// TODO: Only check disjoint-ness between newly explored verticies
impl Solvable for Cube2 {
    const INPUT_FILE_NAME: &'static str = "tubaitu_input_file";

    fn read_from_slate() -> Result<Self, Box<dyn Error>> {
        let input = fs::read_to_string(Cube2::INPUT_FILE_NAME)?;
        read_tubaitu_from_string(&input)
    }
    fn make_move(&mut self, m: &Move) {
        match m.side {
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
            Move::new("R"), Move::new("F"), Move::new("U"),
            Move::new("L'"), Move::new("B'"), Move::new("D'")
        ])
    }
    fn write_blank_slate() -> Result<(), Box<dyn Error>> {
        let template =
"   ┏━━┓
   ┃XX┃
   ┃XX┃
┏━━╋━━╋━━┳━━┓
┃XX┃XX┃XX┃XX┃
┃XX┃XX┃XX┃XX┃
┗━━╋━━╋━━┻━━┛
   ┃XX┃
   ┃XX┃
   ┗━━┛";

        let mut file = File::create(Self::INPUT_FILE_NAME)?;
        file.write(template.as_bytes())?;

        Ok(())

    }

}

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
