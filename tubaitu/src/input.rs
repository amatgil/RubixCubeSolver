use std::ops::Index;

use crate::*;


#[derive(Default, Debug, Clone, Copy)]
pub struct TubaiStickers {
    pub right: TubaiStickerFace,
    pub left: TubaiStickerFace,
    pub front: TubaiStickerFace,
    pub back: TubaiStickerFace,
    pub top: TubaiStickerFace,
    pub down: TubaiStickerFace,
}

/// Represents the four colors on a face. The ordering of the array is: counterclockwise, starting from the "top left", where the top left is the sticker that's at the top left when the cube is rotated the least. For example:

/// | Face   | Moves to turn it into the front's top left | Piece number | Side number |
/// |--------|-----------------------------------------------------|
/// | Front  | -  | 3 | 1 |
/// | Back   | 2L | 6 | 4 |
/// | Top:   | L  | 2 | 2 |
/// | Down   | L' | 7 | 5 |
/// | Right: | U  | 0 | 0 |
/// | Left:  | U' | 2 | 3 |
#[derive(Default, Debug, Clone, Copy)]
pub struct TubaiStickerFace (pub [Color; 4]);

impl Index<usize> for TubaiStickerFace {
    type Output = Color;
    fn index(&self, index: usize) -> &Self::Output {
	&self.0[index]
    }
}

impl Cube2 {
    pub fn from_stickers(s: TubaiStickers) -> Cube2 {
	// Piece seq is: right, front, top, left, back, down

	let p_000: Piece = {
	    let top = s.top[2];
	    let front = s.front[3];
	    Piece { rotation: PieceRotation::from_color_pair(top, front) }
	};
	let p_001: Piece = { 
	    let top = s.top[3];
	    let back = s.back[2];
	    Piece { rotation: PieceRotation::from_color_pair(top, back.opposite()) }
	};
	let p_010: Piece = {
	    let top = s.top[0];
	    let back = s.back[1];
	    Piece { rotation: PieceRotation::from_color_pair(top, back.opposite()) }
	};
	let p_011: Piece = {
	    let top = s.top[1];
	    let front = s.front[0];
	    Piece { rotation: PieceRotation::from_color_pair(top, front) }
	};
	let p_100: Piece = { 
	    let down = s.down[3];
	    let front = s.front[2];
	    Piece { rotation: PieceRotation::from_color_pair(down.opposite(), front) }
	};
	let p_101: Piece = { 
	    let down = s.down[2];
	    let back = s.back[3];
	    Piece { rotation: PieceRotation::from_color_pair(down.opposite(), back.opposite()) }
	};
	let p_110: Piece = { 
	    let down = s.down[1];
	    let back = s.back[0];
	    Piece { rotation: PieceRotation::from_color_pair(down.opposite(), back.opposite()) }
	};
	let p_111: Piece = { 
	    let down = s.down[0];
	    let front = s.front[1];
	    Piece { rotation: PieceRotation::from_color_pair(down.opposite(), front) }
	};

	Cube2 { pieces: [ p_000, p_001, p_010, p_011, p_100, p_101, p_110, p_111  ] }
    }
}

