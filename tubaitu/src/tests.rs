use shared::{Color, Move, Solvable};

use crate::{cycle_items, cycle_items_old, cycle_items_safe, cycle_items_unchecked, Cube2, Piece, PieceRotation, TubaiStickerFace, TubaiStickers};


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
}

#[test]
fn redundant_move_right() {
    let mut cube = Cube2::default();
    cube.make_move(&Move::new("R"));
    cube.make_move(&Move::new("R"));
    cube.make_move(&Move::new("R"));
    cube.make_move(&Move::new("R"));
    assert_eq!(cube, Cube2::default());
}

#[test]
fn redundant_move_up() {
    let mut cube = Cube2::default();
    cube.make_move(&Move::new("U"));
    cube.make_move(&Move::new("U"));
    cube.make_move(&Move::new("U"));
    cube.make_move(&Move::new("U"));
    assert_eq!(cube, Cube2::default());
}

#[test]
fn redundant_move_double_up() {
    let mut cube = Cube2::default();
    cube.make_move(&Move::new("U"));
    cube.make_move(&Move::new("U"));
    cube.make_move(&Move::new("U'"));
    cube.make_move(&Move::new("U'"));
    assert_eq!(cube, Cube2::default());
}


#[test]
fn stickers_solved_input() {
    use Color as C;
    let mut s = TubaiStickers::default();

    s.right  = TubaiStickerFace([C::Orange, C::Orange, C::Orange, C::Orange]);
    s.left   = TubaiStickerFace([C::Red, C::Red, C::Red, C::Red]);
    s.top    = TubaiStickerFace([C::Yellow, C::Yellow, C::Yellow, C::Yellow]);
    s.down   = TubaiStickerFace([C::White, C::White, C::White, C::White]);
    s.front  = TubaiStickerFace([C::Green, C::Green, C::Green, C::Green]);
    s.back   = TubaiStickerFace([C::Blue, C::Blue, C::Blue, C::Blue]);

    let test_cube = Cube2::from_stickers(s);
    let solved_cube = Cube2 { pieces: [Piece { rotation: PieceRotation::WO }; 8] };
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
    righted_cube.make_move(&Move::new("R"));
    dbg!(righted_cube);

    assert!(test_cube == righted_cube)
}
