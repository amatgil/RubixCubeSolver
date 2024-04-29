use crate::*;

pub fn is_solved(c: &Cube2) -> bool {
    c.pieces.iter().fold((true, &c.pieces[0]), |(acc_b, acc_c), x| (acc_b && acc_c == x, x) ).0
}

#[test]
fn basic_is_solved_test() {
    let solved_piece = Piece { rotation: PieceRotation::WO };
    let mut cube = Cube2 { pieces: [solved_piece; 8] };

    assert!(is_solved(&cube));
    cube.make_move(Move::R);
    assert!(!is_solved(&cube));
}


/// Takes in two cubes. Returns a sequence of moves that will turn the left one into the right one
/// Not optimized for efficiency
fn reorient_together(a: Cube2, b: Cube2) -> Option<Vec<Move>> {
    for mut o in get_orientation_generators() {
        for mut r in get_rotation_generators() {
            let mut alternate_cube = a;
            for m1 in &mut *o { alternate_cube.make_move(*m1) }
            for m2 in &r { alternate_cube.make_move(*m2) }
            if alternate_cube.pieces == b.pieces {
                o.append(&mut r);
                return Some(o);
            }
        }
    }
    None
}

#[test]
fn only_right_solve() {
    let mut cube = Cube2::scramble(&vec![Move::R].into());
    for m in cube.solve() { cube.make_move(m); }

    assert_eq!(cube, Cube2::default());
}

#[test]
fn only_left_solve() {
    let mut cube = Cube2::scramble(&vec![Move::L].into());
    for m in cube.solve() { cube.make_move(m); }

    assert_eq!(cube, Cube2::default());
}

#[test]
fn double_up_solve() {
    let mut cube = Cube2::scramble(&vec![
	Move::U,
	Move::U,
    ].into());
    for m in cube.solve() { cube.make_move(m); }

    assert_eq!(cube, Cube2::default());
}

#[test]
fn back_up_solve() {
    let mut cube = Cube2::scramble(&vec![
	Move::B,
	Move::U,
    ].into());
    for m in cube.solve() { cube.make_move(m); }

    assert_eq!(cube, Cube2::default());
}

#[test]
fn redundant_solve() {
    let mut cube = Cube2::scramble(&vec![
	Move::U,
	Move::U,
	Move::U,
    ].into());
    for m in cube.solve() { cube.make_move(m); }

    assert_eq!(cube, Cube2::default());
}

#[test]
fn opposite_solve() {
    let mut cube = Cube2::scramble(&vec![
	Move::L,
	Move::R,
    ].into());
    for m in cube.solve() { cube.make_move(m); }

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
    for m in cube.solve() { cube.make_move(m); }

    assert_eq!(cube, Cube2::default());
}

#[test]
fn reorientation() {
    let a = Cube2::default();
    let mut b = Cube2::default();
    b.make_move(Move::R);
    b.make_move(Move::L);

    let answer = vec![Move::R, Move::L];
    assert_eq!(answer, reorient_together(a, b).unwrap());
}

#[test]
fn reorientation2() {
    let a = Cube2::default();
    let mut b = Cube2::default();
    b.make_move(Move::U);
    b.make_move(Move::U);
    b.make_move(Move::D);
    b.make_move(Move::D);

    let answer = vec![Move::U, Move::D, Move::U, Move::D];
    assert_eq!(answer, reorient_together(a, b).unwrap());
}
