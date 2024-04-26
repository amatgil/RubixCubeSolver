use std::{env, process::exit, time::Instant};

use tribaitri::*;
use shared::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(option) = args.get(1) else {
	println!("[ERROR]: Missing argument: `gen` to (re-)generate template file, `solve` to read from it and output solving steps or `rand` to generate a random scramble and solve it");
	exit(1);
    };

    match &**option {
        "testing" => {
            let mut cube = Cube3::default();
            println!("{cube}");
            cube.make_move(&Move::new("U"));
            println!("{cube}");
            cube.make_move(&Move::new("U"));
            println!("{cube}");
            cube.make_move(&Move::new("U"));
            println!("{cube}");
            cube.make_move(&Move::new("U"));
            println!("{cube}");
        },
	"rand" => {
	    let scramble_length = 3;
	    println!("[INFO]: Generating random cube (n={scramble_length})...");
	    let scrambling_instant = Instant::now();
	    let (mut cube, scramble) = Cube3::random_scramble(scramble_length);
	    let time_taken_to_scramble = scrambling_instant.elapsed();
	    println!("[INFO]: Scrambling took: {}ms ({}μs)", time_taken_to_scramble.as_millis(), time_taken_to_scramble.as_micros());
	    print!("[INFO]: Scramble is: ");
	    println!("{scramble}");
	    print!("[INFO]: (Uncompressed: [ "); for m in &scramble.0 { print!("{m} "); } println!("])");

	    println!("[INFO]: Solving...");
	    println!("Scramble to solve:\n{cube}");

            let starting_instant = Instant::now();
            let r = cube.solve();
            let time_taken = starting_instant.elapsed();

	    for m in &r.0 { cube.make_move(m) }
	    println!("Final state:\n{cube}");

            println!();

	    println!("[RESULT]: Solving time was: {}ms ({}μs)", time_taken.as_millis(), time_taken.as_micros());
            println!("[RESULT]: Final solution is: {r}");
            print!("[INFO]: Uncompressed solution: [ "); for m in &r.0 { print!("{m} "); } println!("]");

            println!();
	    
	    println!("[RESULT]: Reverse of solution: {}", r.reversed());
	    print!("[INFO]: Uncompressed reverse: [ "); for m in r.0.iter().rev() { print!("{} ", m.opposite()); } println!("]");
	},
	"gen" => {
            todo!("Encara no ho he implementat :(");
	},
        "solve" => {
            todo!("El 3x3 encara no té l'entrada feta, intenti-ho amb `rand`");
	}
	o => {
	    println!("[ERROR]: option `{o} not recognized. Please use `gen`, `solve` or `rand`");
	}
    }
}
