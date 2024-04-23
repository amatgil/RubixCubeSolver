use std::{borrow::Cow, error::Error, fs::{self, File}, io::{self, Read, Write}, ops::Index};

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

impl Cube {
    pub fn from_stickers(s: Stickers) -> Cube {
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

	Cube { pieces: [ p_000, p_001, p_010, p_011, p_100, p_101, p_110, p_111  ] }
    }
}

pub const INPUT_FILE_NAME: &str = "tubaitu_input_file";
pub fn write_blank_slate() -> Result<(), Box<dyn Error>> {
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

    let mut file = File::create(INPUT_FILE_NAME)?;
    file.write(template.as_bytes())?;

    Ok(())

}


fn get_next_color(input: &mut impl Iterator<Item = char>, error_s: String) -> Result<Color, Box<dyn Error>> {
    let c = input.next().ok_or(error_s.clone())?;

    let col = Color::from(c);
    col.ok_or(format!("{c} is not a valid color").into())
}

fn skip_n_chars(input: &mut impl Iterator<Item = char>, n: usize, e: String) -> Result<(), String>{
    for _ in 0..n { input.next().ok_or(e.clone())?; }
    Ok(())
}

pub fn read_from_input_file() -> Result<Cube, Box<dyn Error>> {
    let input = fs::read_to_string(INPUT_FILE_NAME)?;
    read_from_string(&input)
}
fn read_from_string(input: &str) -> Result<Cube, Box<dyn Error>> {
    let error_s: Cow<str> = format!("File {INPUT_FILE_NAME} does not represent a cube (valid or non-valid)").into();

    let mut s = Stickers::default();

    let mut input = input.chars();

    skip_n_chars(&mut input, 8, error_s.to_string())?;
    skip_n_chars(&mut input, 4, error_s.to_string())?;

    // TOP FACE
    let top_left  = get_next_color(&mut input, error_s.to_string())?;
    let top_right = get_next_color(&mut input, error_s.to_string())?;

    skip_n_chars(&mut input, 6, error_s.to_string())?;

    let bottom_left  = get_next_color(&mut input, error_s.to_string())?;
    let bottom_right = get_next_color(&mut input, error_s.to_string())?;
    s.top.0 = [top_left, bottom_left, bottom_right, top_right];

    skip_n_chars(&mut input, 17, error_s.to_string())?;


    // Tops
    let left_top_left  = get_next_color(&mut input, error_s.to_string())?;
    let left_top_right = get_next_color(&mut input, error_s.to_string())?;

    skip_n_chars(&mut input, 1, error_s.to_string())?;

    let front_top_left  = get_next_color(&mut input, error_s.to_string())?;
    let front_top_right = get_next_color(&mut input, error_s.to_string())?;

    skip_n_chars(&mut input, 1, error_s.to_string())?;

    let right_top_left  = get_next_color(&mut input, error_s.to_string())?;
    let right_top_right = get_next_color(&mut input, error_s.to_string())?;

    skip_n_chars(&mut input, 1, error_s.to_string())?;

    let back_top_left  = get_next_color(&mut input, error_s.to_string())?;
    let back_top_right = get_next_color(&mut input, error_s.to_string())?;

    // Bottoms
    skip_n_chars(&mut input, 3, error_s.to_string())?;

    let left_bottom_left  = get_next_color(&mut input, error_s.to_string())?;
    let left_bottom_right = get_next_color(&mut input, error_s.to_string())?;

    skip_n_chars(&mut input, 1, error_s.to_string())?;

    let front_bottom_left  = get_next_color(&mut input, error_s.to_string())?;
    let front_bottom_right = get_next_color(&mut input, error_s.to_string())?;

    skip_n_chars(&mut input, 1, error_s.to_string())?;

    let right_bottom_left  = get_next_color(&mut input, error_s.to_string())?;
    let right_bottom_right = get_next_color(&mut input, error_s.to_string())?;

    skip_n_chars(&mut input, 1, error_s.to_string())?;

    let back_bottom_left = get_next_color(&mut input, error_s.to_string())?;
    let back_bottom_right  = get_next_color(&mut input, error_s.to_string())?;

    s.left.0 = [left_top_left, left_bottom_left, left_bottom_right, left_top_right];
    s.right.0 = [right_top_left, right_bottom_left, right_bottom_right, right_top_right];
    s.front.0 = [front_top_left, front_bottom_left, front_bottom_right, front_top_right];
    s.back.0 = [back_bottom_right, back_top_right, back_top_left, back_bottom_left];

    skip_n_chars(&mut input, 20, error_s.to_string())?;

    let down_top_left  = get_next_color(&mut input, error_s.to_string())?;
    let down_top_right = get_next_color(&mut input, error_s.to_string())?;

    skip_n_chars(&mut input, 6, error_s.to_string())?;

    let down_bottom_left  = get_next_color(&mut input, error_s.to_string())?;
    let down_bottom_right = get_next_color(&mut input, error_s.to_string())?;

    s.down.0 = [down_top_left, down_bottom_left, down_bottom_right, down_top_right];

    Ok(Cube::from_stickers(s))
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

    let test_cube = Cube::from_stickers(s);
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

    let test_cube = Cube::from_stickers(s);
    let mut righted_cube = Cube { pieces: [Piece { rotation: PieceRotation::YG }; 8] };
    righted_cube.make_move(&Move::new("R"));
    dbg!(righted_cube);

    assert!(test_cube == righted_cube)
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

    let correct_cube = Cube::scramble(&vec![Move::new("R")].into());
    let r = read_from_string(input).unwrap();
    println!("Comparing: gotten:\n{r}");
    println!("vs expected:\n{correct_cube}");
    assert!(r == correct_cube);
}
