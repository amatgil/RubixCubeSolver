use crate::*;

#[allow(clippy::upper_case_acronyms)]

/// Assigning variants (numbers from the discriminants) based on their three colors. These are always valid, even when unsolved, because the centers never move
/// Numbering goes from 0..7, after which come the edges (Different enum, ThreeByEdge)
#[repr(u8)]
pub enum ThreeByCorner {
    WRB = 0,
    WBO = 1,
    WOG = 2,
    WGR = 3,
    YBR = 4,
    YBO = 5,
    YOG = 6,
    YRG = 7,
}

/// Assigning variants (numbers from the discriminants) based on their two colors. These are always valid, even when unsolved, because the centers never move.
/// Numbering starts at 8 to avoid collisions with the corners
#[repr(u8)]
pub enum ThreeByEdge {
    WR = 8, // Start at 8 to not collide with the corner's indexes
    WB = 9,
    WO = 10,
    WG = 11,
    YR = 12,
    YB = 13,
    YO = 14,
    YG = 15,
    OB = 16,
    BR = 17,
    RG = 18,
    GO = 19,
}

pub const RIGHT_EDGE_CYCLE: [ThreeByEdge; 4] = [ThreeByEdge::YB, ThreeByEdge::BR, ThreeByEdge::WB, ThreeByEdge::OB]; // Blue
pub const LEFT_EDGE_CYCLE:  [ThreeByEdge; 4] = [ThreeByEdge::YG, ThreeByEdge::GO, ThreeByEdge::WG, ThreeByEdge::RG]; // Green
pub const FRONT_EDGE_CYCLE: [ThreeByEdge; 4] = [ThreeByEdge::YO, ThreeByEdge::OB, ThreeByEdge::WO, ThreeByEdge::GO]; // Orange
pub const BACK_EDGE_CYCLE:  [ThreeByEdge; 4] = [ThreeByEdge::YR, ThreeByEdge::RG, ThreeByEdge::WR, ThreeByEdge::BR]; // Red
pub const UP_EDGE_CYCLE:    [ThreeByEdge; 4] = [ThreeByEdge::YO, ThreeByEdge::YG, ThreeByEdge::YR, ThreeByEdge::YB]; // Yellow
pub const DOWN_EDGE_CYCLE:  [ThreeByEdge; 4] = [ThreeByEdge::WO, ThreeByEdge::WB, ThreeByEdge::WR, ThreeByEdge::WG]; // White

pub const RIGHT_CORNER_CYCLE: [ThreeByCorner; 4] = [ThreeByCorner::YBO, ThreeByCorner::YOG, ThreeByCorner::YRG, ThreeByCorner::YBR]; 
pub const LEFT_CORNER_CYCLE:  [ThreeByCorner; 4] = [ThreeByCorner::YOG, ThreeByCorner::WOG, ThreeByCorner::WGR, ThreeByCorner::YRG]; 
pub const FRONT_CORNER_CYCLE: [ThreeByCorner; 4] = [ThreeByCorner::YOG, ThreeByCorner::YOG, ThreeByCorner::WBO, ThreeByCorner::WOG]; 
pub const BACK_CORNER_CYCLE:  [ThreeByCorner; 4] = [ThreeByCorner::YBR, ThreeByCorner::YRG, ThreeByCorner::WGR, ThreeByCorner::WRB]; 
pub const UP_CORNER_CYCLE:    [ThreeByCorner; 4] = [ThreeByCorner::YBO, ThreeByCorner::YOG, ThreeByCorner::YRG, ThreeByCorner::YBR]; 
pub const DOWN_CORNER_CYCLE:  [ThreeByCorner; 4] = [ThreeByCorner::WBO, ThreeByCorner::WRB, ThreeByCorner::WGR, ThreeByCorner::WOG]; 


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
    match m {
        Move { side: MoveSide::R, .. } => Cube3::cycle_elements::<20>(&mut cube.pieces, RIGHT_CORNER_CYCLE.map(|v| v as usize), m),
        Move { side: MoveSide::L, .. } => Cube3::cycle_elements::<20>(&mut cube.pieces, LEFT_CORNER_CYCLE.map( |v| v as usize), m),
        Move { side: MoveSide::F, .. } => Cube3::cycle_elements::<20>(&mut cube.pieces, FRONT_CORNER_CYCLE.map(|v| v as usize), m),
        Move { side: MoveSide::B, .. } => Cube3::cycle_elements::<20>(&mut cube.pieces, BACK_CORNER_CYCLE.map( |v| v as usize), m),
        Move { side: MoveSide::U, .. } => Cube3::cycle_elements::<20>(&mut cube.pieces, UP_CORNER_CYCLE.map(   |v| v as usize), m),
        Move { side: MoveSide::D, .. } => Cube3::cycle_elements::<20>(&mut cube.pieces, DOWN_CORNER_CYCLE.map( |v| v as usize), m),
    }

}
