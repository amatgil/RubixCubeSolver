use std::ops::Index;

use crate::*;


#[derive(Default, Debug, Clone, Copy)]
pub struct Stickers {
    pub right: StickerFace,
    pub left: StickerFace,
    pub front: StickerFace,
    pub back: StickerFace,
    pub top: StickerFace,
    pub down: StickerFace,
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
pub struct StickerFace (pub [Color; 4]);

impl Index<usize> for StickerFace {
    type Output = Color;
    fn index(&self, index: usize) -> &Self::Output {
	&self.0[index]
    }
}

pub fn cube_from_stickers(s: Stickers) -> Cube {
    // Piece seq is: right, front, top, left, back, down

    let p_000: Piece = {
	let down = s.down[0];
	let front = s.front[1];
	Piece { rotation: PieceRotation::from_color_pair(down.opposite(), front) }
    };
    let p_001: Piece = { 
	let down = s.down[1];
	let back = s.back[0];
	Piece { rotation: PieceRotation::from_color_pair(down.opposite(), back.opposite()) }
    };
    let p_010: Piece = {
	let top = s.top[1];
	let front = s.front[0];
	Piece { rotation: PieceRotation::from_color_pair(top, front) }
    };

    let p_011: Piece = {
	let top = s.top[0];
	let back = s.back[1];
	Piece { rotation: PieceRotation::from_color_pair(top, back.opposite()) }
    };

    let p_110: Piece = { 
	let top = s.top[2];
	let front = s.front[3];
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

    let p_111: Piece = { 
	let top = s.top[3];
	let back = s.back[2];
	Piece { rotation: PieceRotation::from_color_pair(top, back.opposite()) }
    };

    Cube { pieces: [ p_110, p_111, p_011, p_010, p_100, p_101, p_001, p_000 ] }
}

#[test]
fn stickers_solved_input() {
    use Color as C;
    let mut s = Stickers::default();

    s.right  = StickerFace([C::Orange, C::Orange, C::Orange, C::Orange]);
    s.left   = StickerFace([C::Red, C::Red, C::Red, C::Red]);
    s.top    = StickerFace([C::Yellow, C::Yellow, C::Yellow, C::Yellow]);
    s.down   = StickerFace([C::White, C::White, C::White, C::White]);
    s.front  = StickerFace([C::Green, C::Green, C::Green, C::Green]);
    s.back   = StickerFace([C::Blue, C::Blue, C::Blue, C::Blue]);

    let test_cube = cube_from_stickers(s);
    let solved_cube = Cube { pieces: [Piece { rotation: PieceRotation::WO }; 8] };
    dbg!(test_cube, solved_cube);
    assert!(test_cube == solved_cube)
}

#[test]
fn stickers_afterright_input() {
    use Color as C;
    let mut s = Stickers::default();

    s.right  = StickerFace([C::Orange, C::Orange, C::Orange, C::Orange]);
    s.left   = StickerFace([C::Red, C::Red, C::Red, C::Red]);
    s.top    = StickerFace([C::Yellow, C::Yellow, C::Green, C::Green]);
    s.down   = StickerFace([C::White, C::White, C::Blue, C::Blue]);
    s.front  = StickerFace([C::Green, C::Green, C::White, C::White]);
    s.back   = StickerFace([C::Blue, C::Blue, C::Yellow, C::Yellow]);

    let test_cube = cube_from_stickers(s);
    let mut righted_cube = Cube { pieces: [Piece { rotation: PieceRotation::YG }; 8] };
    righted_cube.make_move(&Move::new("R"));
    dbg!(righted_cube);

    assert!(test_cube == righted_cube)
}
