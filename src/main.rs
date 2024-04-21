use tubaitu::*;

fn main() {
    use Color as C;

    //s.front  = StickerFace([C::Yellow, C::Green, C::Yellow, C::Blue]);
    //s.right  = StickerFace([C::White, C::Red, C::Orange, C::Blue]);
    //s.left   = StickerFace([C::Green, C::Red, C::Yellow, C::Orange]);
    //s.top    = StickerFace([C::White, C::Green, C::Red, C::Orange]);
    //s.down   = StickerFace([C::Red, C::Green, C::White, C::Blue]);
    //s.back   = StickerFace([C::White, C::Orange, C::Yellow, C::Blue]);

    let scrambling_moves = &vec![
	Move::new("R"),
	Move::new("U"),
	Move::new("R'"),
	Move::new("U'"),
	Move::new("R'"),
	Move::new("F"),
	Move::new("R"),
	Move::new("R"),
	Move::new("U'"),
	Move::new("R'"),
	Move::new("U'"),
	Move::new("R"),
	Move::new("U"),
	Move::new("R'"),
	Move::new("F'"),
    ];
//R U R' U R U R' U 

    let cube = Cube::scramble(scrambling_moves);

    //println!("{cube}");

    let r = solve(cube);

    println!("Comprovant correctesa");
    let mut checking_cube = Cube::scramble(scrambling_moves);
    for m in &r { checking_cube.make_move(&m) }
    println!("Final cube: {checking_cube}");

    print_solution(&r);
    println!("And its pieces are: {:?}", checking_cube.pieces.map(|p| p.rotation));
}
