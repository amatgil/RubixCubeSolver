
use tubaitu::*;

fn main() {
    let moves: Vec<Move> = vec![
        Move::new("R"),
        Move::new("U"),
        Move::new("F"),
    ];
    draw_sequence("output_", &Cube::default(), moves, 3).unwrap();
}