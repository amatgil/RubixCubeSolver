
use tubaitu::*;
use tribaitri::*;

//use rayon::prelude::*;

fn main() {
    let attempts_per_size = 5;
    let min_scramble_size = 2;
    let max_scramble_size = 20;

    for n in min_scramble_size..max_scramble_size {
        (0..attempts_per_size).for_each(|i| {
            let (cube, _) = Cube3::random_scramble(n);
            let r = cube.solve();
            println!("{n} {}", r.len());
            eprintln!("Solved for {n}, attempt {i}");
        })
    }
}
