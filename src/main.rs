use tubaitu::*;

fn main() {
    use Color as C;
    let mut s = Stickers::default();

    s.front  = StickerFace([C::Orange, C::Yellow, C::Green, C::Red]);
    s.right  = StickerFace([C::Green, C::Orange, C::White, C::Red]);
    s.left   = StickerFace([C::Orange, C::Green, C::Blue, C::Yellow]);
    s.top    = StickerFace([C::White, C::Blue, C::Yellow, C::Green]);
    s.down   = StickerFace([C::Red, C::Yellow, C::Red, C::White]);
    s.back   = StickerFace([C::Orange, C::Blue, C::White, C::Blue]);

    let cube = cube_from_stickers(s);

    let r = solve(cube);

    print!("Solution is:  ");
    for m in r {
	print!("{m} ");
    }
    println!()

}
