pub mod declarations;
use std::ops::Deref;

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
#[repr(usize)]
#[derive(Debug, Clone, Copy)]
enum PiecePosition {
    TopRightFront,
    TopRightBack,
    TopLeftBack,
    TopLeftFront,
    BottomRightFront,
    BottomRightBack,
    BottomLeftBack,
    BottomLeftFront,
}

type P = PiecePosition;

// Rotation constants 
const FACE_RIGHT_SEQ_CYCLE: [P; 4] = [P::TopRightBack,    P::BottomRightBack, P::BottomRightFront, P::TopRightFront];
const FACE_LEFT_SEQ_CYCLE: [P; 4]  = [P::TopLeftFront,    P::BottomLeftFront, P::BottomLeftBack,   P::TopLeftBack];
const FACE_UP_SEQ_CYCLE: [P; 4]    = [P::TopLeftFront,    P::TopLeftBack,     P::TopRightBack,     P::TopRightFront];
const FACE_DOWN_SEQ_CYCLE: [P; 4]  = [P::BottomLeftBack,  P::BottomLeftFront, P::BottomRightFront, P::BottomRightBack];
const FACE_FRONT_SEQ_CYCLE: [P; 4] = [P::BottomLeftFront, P::TopLeftFront,    P::TopRightFront,    P::BottomRightFront];
const FACE_BACK_SEQ_CYCLE: [P; 4]  = [P::BottomLeftBack,  P::BottomRightBack, P::TopRightBack,     P::TopLeftBack];

const FACE_RIGHT_SEQ_PRINT: [P; 4] = [P::TopRightFront,   P::TopRightBack,     P::BottomRightFront, P::BottomRightBack];
const FACE_LEFT_SEQ_PRINT: [P; 4]  = [P::TopLeftBack,     P::TopLeftFront,     P::BottomLeftBack,   P::BottomLeftFront];
const FACE_UP_SEQ_PRINT: [P; 4]    = [P::TopLeftBack,     P::TopRightBack,     P::TopLeftFront,     P::TopRightFront];
const FACE_DOWN_SEQ_PRINT: [P; 4]  = [P::BottomLeftFront, P::BottomRightFront, P::BottomLeftBack,   P::BottomRightBack];
const FACE_FRONT_SEQ_PRINT: [P; 4] = [P::TopLeftFront,    P::TopRightFront,    P::BottomLeftFront,  P::BottomRightFront];
const FACE_BACK_SEQ_PRINT: [P; 4]  = [P::TopRightBack,    P::TopLeftBack,      P::BottomRightBack,  P::BottomLeftBack];

impl Solvable for Cube2 {
    fn make_move(&mut self, m: Move) {
        match m.side() {
            MoveSide::R => Self::cycle_elements::<8, 4>(&mut self.pieces, FACE_RIGHT_SEQ_CYCLE.map(|p| *p), m),
            MoveSide::L => Self::cycle_elements::<8, 4>(&mut self.pieces, FACE_LEFT_SEQ_CYCLE .map(|p| *p) , m),
            MoveSide::U => Self::cycle_elements::<8, 4>(&mut self.pieces, FACE_UP_SEQ_CYCLE   .map(|p| *p)   , m),
            MoveSide::B => Self::cycle_elements::<8, 4>(&mut self.pieces, FACE_BACK_SEQ_CYCLE .map(|p| *p) , m),
            MoveSide::F => Self::cycle_elements::<8, 4>(&mut self.pieces, FACE_FRONT_SEQ_CYCLE.map(|p| *p), m),
            MoveSide::D => Self::cycle_elements::<8, 4>(&mut self.pieces, FACE_DOWN_SEQ_CYCLE .map(|p| *p) , m),
        };
    }
    fn moves_of_adjacency() -> Vec<Move> {
        Vec::from([
            Move::R, Move::F, Move::U,
            Move::L, Move::B, Move::D
        ])
    }
}

impl TryFrom<usize> for PiecePosition {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, ()> {
        if value > 7 { return Err(()) }
        else { Ok(unsafe { std::mem::transmute(value) })}
    }

}
impl Deref for PiecePosition {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
