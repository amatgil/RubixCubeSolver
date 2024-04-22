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
    ];

    let cube = Cube::scramble(scrambling_moves);

    let r = solve(cube);

    println!("[INFO]: Checking correctness...");
    let mut checking_cube = Cube::scramble(scrambling_moves);
    for m in &r { checking_cube.make_move(&m) }

    println!("Starting cube:\n{cube}\n");
    println!("Final cube:\n{checking_cube}");
    print_solution(&r);
}
