
use tubaitu::*;
use tribaitri::*;

fn main() {
    let attempts_per_size = 15;
    let min_scramble_size = 2;
    let max_scramble_size = 100;

    for n in min_scramble_size..max_scramble_size {
        for i in 0..attempts_per_size {
            let (cube, _) = Cube2::random_scramble(n);
            let r = cube.solve();
            println!("{n} {}", r.len());
            eprintln!("Solved for {n}, attempt {i}");
        }
    }
}
