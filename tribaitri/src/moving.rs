use crate::*;

/// Assigning variants (numbers from the discriminants) based on their three colors. These are always valid, even when unsolved, because the centers never move
#[repr(u8)]
pub enum ThreeByCorner {
    WRB,
    WBO,
    WOG,
    WGR,
    YBR,
    YBO,
    YOG,
    YRG,
}

/// Assigning variants (numbers from the discriminants) based on their two colors. These are always valid, even when unsolved, because the centers never move
#[repr(u8)]
pub enum ThreeByEdge {
    WR,
    WB,
    WO,
    WG,
    YR,
    YB,
    YO,
    YG,
    OB,
    BR,
    RG,
    GO,
}

const RIGHT_EDGE_CYCLE: [ThreeByEdge; 4] = [ThreeByEdge::YB, ThreeByEdge::BR, ThreeByEdge::WB, ThreeByEdge::OB];
const LEFT_EDGE_CYCLE:  [ThreeByEdge; 4] = [ThreeByEdge::YG, ThreeByEdge::GO, ThreeByEdge::WG, ThreeByEdge::RG];
const FRONT_EDGE_CYCLE: [ThreeByEdge; 4] = [ThreeByEdge::];
const BACK_EDGE_CYCLE:  [ThreeByEdge; 4] = [];
const UP_EDGE_CYCLE:    [ThreeByEdge; 4] = [];
const DOWN_EDGE_CYCLE:  [ThreeByEdge; 4] = [];

/// Note our representation:
/// | Face  | Center Color |
/// |-------|--------------|
/// | Up    | Yellow       |
/// | Down  | White        |
/// | Left  | Green        |
/// | Right | Blue         |
/// | Front | Orange       |
/// | Back  | Red          |
pub(crate) fn make_three_by_three_move(cube: &mut Cube3, m: &Move) {
    // Cycle arestes
    match m {
        Move { side: MoveSide::R, .. } => Cube3::cycle_elements::<20>(&mut cube.pieces, RIGHT_EDGE_CYCLE.map(|v| v as usize), m),
        Move { side: MoveSide::L, .. } => Cube3::cycle_elements::<20>(&mut cube.pieces, LEFT_EDGE_CYCLE.map( |v| v as usize), m),
        Move { side: MoveSide::F, .. } => Cube3::cycle_elements::<20>(&mut cube.pieces, FRONT_EDGE_CYCLE.map(|v| v as usize), m),
        Move { side: MoveSide::B, .. } => Cube3::cycle_elements::<20>(&mut cube.pieces, BACK_EDGE_CYCLE.map( |v| v as usize), m),
        Move { side: MoveSide::U, .. } => Cube3::cycle_elements::<20>(&mut cube.pieces, UP_EDGE_CYCLE.map(   |v| v as usize), m),
        Move { side: MoveSide::D, .. } => Cube3::cycle_elements::<20>(&mut cube.pieces, DOWN_EDGE_CYCLE.map( |v| v as usize), m),
        
    }

    // Cycle edges
}

pub fn cycle_items<T: Clone, const N: usize>(v: &mut [T; N], idxs: [usize; 4]) {
    v.swap(idxs[0], idxs[1]);
    v.swap(idxs[0], idxs[2]);
    v.swap(idxs[0], idxs[3]);
}
