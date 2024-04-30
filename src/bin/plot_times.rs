
use tubaitu::*;
use tribaitri::*;

//use rayon::prelude::*;

fn main() {
    let attempts_per_size = 3;
    let min_scramble_size = 1;
    let max_scramble_size = 14;

    (min_scramble_size..max_scramble_size).into_iter().for_each(|n| {
        (0..attempts_per_size).into_par_iter().for_each(|i| {
            let (cube, _) = Cube3::random_scramble(n);
            let r = cube.solve(false);
            println!("{n} {}", r.1);
            eprintln!("Solved for {n}, attempt {i}");
        })
    })
}
