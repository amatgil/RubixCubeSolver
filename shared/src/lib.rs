
pub mod colors;
use std::ops::Deref;

pub use colors::*;

mod solve;

mod display;

pub use display::*;
pub use solve::Solvable;

pub const FLOAT_EPSILON: f64 = 0.001;

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum Side {
    Right,
    Front,
    Top,
    Left,
    Back,
    Down,
}
type S = Side;

pub const COLOR_RIGHT_SEQ: [S; 4] = [S::Front, S::Top,   S::Back,  S::Down];
pub const COLOR_LEFT_SEQ: [S; 4]  = [S::Down,  S::Back,  S::Top,   S::Front];
pub const COLOR_UP_SEQ: [S; 4]    = [S::Front, S::Left,  S::Back,  S::Right];
pub const COLOR_DOWN_SEQ: [S; 4]  = [S::Right, S::Back,  S::Left,  S::Front];
pub const COLOR_FRONT_SEQ: [S; 4] = [S::Top,   S::Right, S::Down,  S::Left];
pub const COLOR_BACK_SEQ: [S; 4]  = [S::Left,  S::Down,  S::Right, S::Top];

 
/// A move, internally represented by a single u8 using bit magic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move(u8);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveSide {
    R, L,
    F, B,
    U, D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Piece {
    pub rotation: PieceRotation,
}

/// Stored as [top color][front color], which uniquely defines a rotation (because the cross product isn't commutative!)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PieceRotation {
    WR, WO, WG, WB,
    RW, BW, OW, GW,
    RY, BY, OY, GY,
    OG, GO, OB, BO,
    RG, GR, RB, BR,
    YR, YB, YG,
    #[default] YO,
}

impl Deref for Side {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        // SAFETY: The enum is repr(usize): this is not UB
        unsafe { std::mem::transmute(self) }
    }
}

impl Move {
    pub const R:  Move = Self(0);
    pub const RP: Move = Self(1);
    pub const F:  Move = Self(2);
    pub const FP: Move = Self(3);
    pub const U:  Move = Self(4);
    pub const UP: Move = Self(5);
    pub const L:  Move = Self(6);
    pub const LP: Move = Self(7);
    pub const B:  Move = Self(8);
    pub const BP: Move = Self(9);
    pub const D:  Move = Self(10);
    pub const DP: Move = Self(11);

    pub fn opposite(&self) -> Self {
        Self(self.0 ^ 1)
    }
    pub fn is_prime(&self) -> bool {
        (self.0 & 1) != 0
    }
    pub fn side(&self) -> MoveSide {
        match *self {
            Self::R | Self::RP => MoveSide::R,
            Self::F | Self::FP => MoveSide::F,
            Self::U | Self::UP => MoveSide::U,
            Self::L | Self::LP => MoveSide::L,
            Self::B | Self::BP => MoveSide::B,
            Self::D | Self::DP => MoveSide::D,
            _ => unreachable!(), // TODO: Use _unchecked when tests pass
        }
    }
}


#[test]
fn opposite_moves() {
    assert_eq!(Move::R.opposite(), Move::RP);
    assert_eq!(Move::RP.opposite(), Move::R);
    assert_eq!(Move::L.opposite(), Move::LP);
    assert_eq!(Move::LP.opposite(), Move::L);
    assert_eq!(Move::F.opposite(), Move::FP);
    assert_eq!(Move::FP.opposite(), Move::F);
    assert_eq!(Move::U.opposite(), Move::UP);
    assert_eq!(Move::UP.opposite(), Move::U);
}

#[test]
fn primeness() {
    assert!(!Move::R.is_prime());
    assert!(!Move::L.is_prime());
    assert!(!Move::F.is_prime());
    assert!(!Move::B.is_prime());
    assert!(!Move::U.is_prime());
    assert!(!Move::D.is_prime());
    assert!( Move::RP.is_prime());
    assert!( Move::LP.is_prime());
    assert!( Move::FP.is_prime());
    assert!( Move::BP.is_prime());
    assert!( Move::UP.is_prime());
    assert!( Move::DP.is_prime());
}
