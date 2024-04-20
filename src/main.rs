use tubaitu::*;

fn main() {
    let piece = Piece::new(['G', 'O', 'W', 'B', 'R', 'Y']);
    let mut cube: Cube = Cube {
        pieces: [piece; 8],
    };

    let scramble = vec![
	Move::new("L"),
	Move::new("U"),
	Move::new("U"),
	Move::new("B"),
    ];

    //cube.make_move(&Move::new("R"));
    for m in &scramble { cube.make_move(m) }

    let r = solve(cube);

    print!("The solution to scramble \"");
    for m in scramble { print!("{m}") }
    print!("\" is \"");
    for m in r { print!("{m}") }
    println!("\"");
}
