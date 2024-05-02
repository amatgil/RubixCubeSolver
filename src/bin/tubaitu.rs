use std::{env, process::exit, time::Instant};

use tubaitu::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(option) = args.get(1) else {
	println!("[ERROR]: Missing argument: `gen` to (re-)generate template file, `solve` to read from it and output solving steps or `rand` to generate a random scramble and solve it");
	exit(1)
    };

    match &**option {
	"rand" => Cube2::solve_random(20, true),
	"gen" => Cube2::write_blank_slate().unwrap(),
	"solve" => Cube2::solve_pretty(),
	o => println!("[ERROR]: option `{o} not recognized. Please use `gen`, `solve` or `rand`"),
    }
}
