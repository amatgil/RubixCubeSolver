use std::{borrow::Cow, error::Error, ops::Index};

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

fn get_next_color(input: &mut impl Iterator<Item = char>, error_s: &str) -> Result<Color, Box<dyn Error>> {
    let c = input.next().ok_or(error_s.to_string())?;

    let col = Color::from(c);
    col.ok_or(format!("{c} is not a valid color").into())
}

fn skip_n_chars(input: &mut impl Iterator<Item = char>, n: usize, e: &str) -> Result<(), String>{
    for _ in 0..n { input.next().ok_or(e.to_string())?; }
    Ok(())
}

pub fn read_tubaitu_from_string(input: &str) -> Result<Cube2, Box<dyn Error>> {
    let error_s: Cow<str> = format!("File {} does not represent a cube (valid or non-valid)", Cube2::INPUT_FILE_NAME).into();

    let mut s = TubaiStickers::default();

    let mut input = input.chars();

    skip_n_chars(&mut input, 8, &error_s)?;
    skip_n_chars(&mut input, 4, &error_s)?;

    // TOP FACE
    let top_left  = get_next_color(&mut input, &error_s)?;
    let top_right = get_next_color(&mut input, &error_s)?;

    skip_n_chars(&mut input, 6, &error_s)?;

    let bottom_left  = get_next_color(&mut input, &error_s)?;
    let bottom_right = get_next_color(&mut input, &error_s)?;
    s.top.0 = [top_left, bottom_left, bottom_right, top_right];

    skip_n_chars(&mut input, 17, &error_s)?;


    // Tops
    let left_top_left  = get_next_color(&mut input, &error_s)?;
    let left_top_right = get_next_color(&mut input, &error_s)?;

    skip_n_chars(&mut input, 1, &error_s)?;

    let front_top_left  = get_next_color(&mut input, &error_s)?;
    let front_top_right = get_next_color(&mut input, &error_s)?;

    skip_n_chars(&mut input, 1, &error_s)?;

    let right_top_left  = get_next_color(&mut input, &error_s)?;
    let right_top_right = get_next_color(&mut input, &error_s)?;

    skip_n_chars(&mut input, 1, &error_s)?;

    let back_top_left  = get_next_color(&mut input, &error_s)?;
    let back_top_right = get_next_color(&mut input, &error_s)?;

    // Bottoms
    skip_n_chars(&mut input, 3, &error_s)?;

    let left_bottom_left  = get_next_color(&mut input, &error_s)?;
    let left_bottom_right = get_next_color(&mut input, &error_s)?;

    skip_n_chars(&mut input, 1, &error_s)?;

    let front_bottom_left  = get_next_color(&mut input, &error_s)?;
    let front_bottom_right = get_next_color(&mut input, &error_s)?;

    skip_n_chars(&mut input, 1, &error_s)?;

    let right_bottom_left  = get_next_color(&mut input, &error_s)?;
    let right_bottom_right = get_next_color(&mut input, &error_s)?;

    skip_n_chars(&mut input, 1, &error_s)?;

    let back_bottom_left = get_next_color(&mut input, &error_s)?;
    let back_bottom_right  = get_next_color(&mut input, &error_s)?;

    s.left.0 = [left_top_left, left_bottom_left, left_bottom_right, left_top_right];
    s.right.0 = [right_top_left, right_bottom_left, right_bottom_right, right_top_right];
    s.front.0 = [front_top_left, front_bottom_left, front_bottom_right, front_top_right];
    s.back.0 = [back_bottom_right, back_top_right, back_top_left, back_bottom_left];

    skip_n_chars(&mut input, 20, &error_s)?;

    let down_top_left  = get_next_color(&mut input, &error_s)?;
    let down_top_right = get_next_color(&mut input, &error_s)?;

    skip_n_chars(&mut input, 6, &error_s)?;

    let down_bottom_left  = get_next_color(&mut input, &error_s)?;
    let down_bottom_right = get_next_color(&mut input, &error_s)?;

    s.down.0 = [down_top_left, down_bottom_left, down_bottom_right, down_top_right];

    Ok(Cube2::from_stickers(s))
}


#[test]
fn from_string_right() {
    let input = 
"   ┏━━┓
   ┃WB┃
   ┃WB┃
┏━━╋━━╋━━┳━━┓
┃RR┃BY┃OO┃WG┃
┃RR┃BY┃OO┃WG┃
┗━━╋━━╋━━┻━━┛
   ┃YG┃
   ┃YG┃
   ┗━━┛";

    let correct_cube = Cube2::scramble(&vec![Move::new("R")].into());
    /*let r = read_from_string(input).unwrap();
    println!("Comparing: gotten:\n{r}");
    println!("vs expected:\n{correct_cube}");
    assert!(r == correct_cube);*/
}
