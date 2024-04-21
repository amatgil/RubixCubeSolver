use tubaitu::*;

fn main() {
    use Color as C;

    //s.front  = StickerFace([C::Yellow, C::Green, C::Yellow, C::Blue]);
    //s.right  = StickerFace([C::White, C::Red, C::Orange, C::Blue]);
    //s.left   = StickerFace([C::Green, C::Red, C::Yellow, C::Orange]);
    //s.top    = StickerFace([C::White, C::Green, C::Red, C::Orange]);
    //s.down   = StickerFace([C::Red, C::Green, C::White, C::Blue]);
    //s.back   = StickerFace([C::White, C::Orange, C::Yellow, C::Blue]);

    let cube = Cube::scramble(&vec![
	Move::new("R"),
	Move::new("R"),
	Move::new("U"),
	Move::new("U"),
	Move::new("D"),
    ]);

    let r = solve(cube);
    print_solution(&r);
}
