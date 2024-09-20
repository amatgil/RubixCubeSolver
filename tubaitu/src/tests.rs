use shared::{Color, Move, Solvable};

use crate::{Cube2, Piece, PieceRotation, TubaiStickerFace, TubaiStickers};

fn is_solved(c: &Cube2) -> bool {
    c.pieces.iter().all(|x| x == &c.pieces[0]) // All equal head
}

/* 
#[test]
fn cycling_test() {
    let t1 = [1, 2, 3, 4, 5];
    let idx = [0, 1, 2, 3];

    let mut a = t1.clone();
    let mut b = t1.clone();
    cycle_items(&mut a, idx);
    cycle_items_old(&mut b, idx);

    assert_eq!(a, b);
}

#[test]
fn cycling_test_unchecked() {
    let t1 = [1, 2, 3, 4, 5];
    let idx = [0, 1, 2, 3];

    let mut a = t1.clone();
    let mut b = t1.clone();
    cycle_items_safe(&mut a, idx);
    cycle_items_unchecked(&mut b, idx);

    assert_eq!(a, b);
}*/

#[test]
fn redundant_move_right() {
    let mut cube = Cube2::default();
    cube.make_move(Move::R);
    cube.make_move(Move::R);
    cube.make_move(Move::R);
    cube.make_move(Move::R);
    assert_eq!(cube, Cube2::default());
}

#[test]
fn redundant_move_up() {
    let mut cube = Cube2::default();
    cube.make_move(Move::U);
    cube.make_move(Move::U);
    cube.make_move(Move::U);
    cube.make_move(Move::U);
    assert_eq!(cube, Cube2::default());
}

#[test]
fn redundant_move_double_up() {
    let mut cube = Cube2::default();
    cube.make_move(Move::U);
    cube.make_move(Move::U);
    cube.make_move(Move::U);
    cube.make_move(Move::U);
    assert_eq!(cube, Cube2::default());
}


#[test]
fn stickers_solved_input() {
    use Color as C;
    let mut s = TubaiStickers::default();

    s.top    = TubaiStickerFace([C::Yellow, C::Yellow, C::Yellow, C::Yellow]);
    s.front = TubaiStickerFace([C::Orange, C::Orange, C::Orange, C::Orange]);
    s.back   = TubaiStickerFace([C::Red, C::Red, C::Red, C::Red]);
    s.down   = TubaiStickerFace([C::White, C::White, C::White, C::White]);
    s.left  = TubaiStickerFace([C::Green, C::Green, C::Green, C::Green]);
    s.right   = TubaiStickerFace([C::Blue, C::Blue, C::Blue, C::Blue]);

    let test_cube = Cube2::from_stickers(s);
    let solved_cube = Cube2 { pieces: [Piece { rotation: PieceRotation::YO }; 8] };
    dbg!(test_cube, solved_cube);
    assert!(test_cube == solved_cube)
}

#[test]
fn stickers_afterright_input() {
    use Color as C;
    let mut s = TubaiStickers::default();

    s.right  = TubaiStickerFace([C::Orange, C::Orange, C::Orange, C::Orange]);
    s.left   = TubaiStickerFace([C::Red, C::Red, C::Red, C::Red]);
    s.top    = TubaiStickerFace([C::Yellow, C::Yellow, C::Green, C::Green]);
    s.down   = TubaiStickerFace([C::White, C::White, C::Blue, C::Blue]);
    s.front  = TubaiStickerFace([C::Green, C::Green, C::White, C::White]);
    s.back   = TubaiStickerFace([C::Blue, C::Blue, C::Yellow, C::Yellow]);

    let test_cube = Cube2::from_stickers(s);
    let mut righted_cube = Cube2 { pieces: [Piece { rotation: PieceRotation::YG }; 8] };
    righted_cube.make_move(Move::R);
    dbg!(righted_cube);

    assert!(test_cube == righted_cube)
}


#[test]
fn basic_is_solved_test() {
    let solved_piece = Piece { rotation: PieceRotation::WO };
    let mut cube = Cube2 { pieces: [solved_piece; 8] };

    assert!(is_solved(&cube));
    cube.make_move(Move::R);
    assert!(!is_solved(&cube));
}


#[test]
fn only_right_solve() {
    let mut cube = Cube2::scramble(&vec![Move::R].into());
    for m in cube.solve(true, None) { cube.make_move(m); }

    assert_eq!(cube, Cube2::default());
}

#[test]
fn only_left_solve() {
    let mut cube = Cube2::scramble(&vec![Move::L].into());
    for m in cube.solve(true, None) { cube.make_move(m); }

    assert_eq!(cube, Cube2::default());
}

#[test]
fn double_up_solve() {
    let mut cube = Cube2::scramble(&vec![
	Move::U,
	Move::U,
    ].into());
    for m in cube.solve(true, None) { cube.make_move(m); }

    assert_eq!(cube, Cube2::default());
}

#[test]
fn back_up_solve() {
    let mut cube = Cube2::scramble(&vec![
	Move::B,
	Move::U,
    ].into());
    for m in cube.solve(true, None) { cube.make_move(m); }

    assert_eq!(cube, Cube2::default());
}

#[test]
fn redundant_solve() {
    let mut cube = Cube2::scramble(&vec![
	Move::U,
	Move::U,
	Move::U,
    ].into());
    for m in cube.solve(true, None) { cube.make_move(m); }

    assert_eq!(cube, Cube2::default());
}

#[test]
fn opposite_solve() {
    let mut cube = Cube2::scramble(&vec![
	Move::L,
	Move::R,
    ].into());
    for m in cube.solve(true, None) { cube.make_move(m); }

    assert_eq!(cube, Cube2::default());
}

#[test]
fn complicated_solve() {
    let mut cube = Cube2::scramble(&vec![
	Move::R,
	Move::U,
	Move::L,
	Move::D,
	Move::F,
    ].into());
    for m in cube.solve(true, None) { cube.make_move(m); }

    assert_eq!(cube, Cube2::default());
}
