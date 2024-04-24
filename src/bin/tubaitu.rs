use std::{env, process::exit};

use tubaitu::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(option) = args.get(1) else {
	println!("[ERROR]: Missing argument: `gen` to (re-)generate template file, `solve` to read from it and output solving steps or `rand` to generate a random scramble and solve it");
	exit(1);
    };

    match &**option {
	"rand" => {
	    let scramble_length = 10;
	    println!("[INFO]: Generating random cube (n={scramble_length})...");
	    let (mut cube, scramble) = Cube::random_scramble(scramble_length);
	    println!("[INFO]: Generated cube:");

	    print!("[INFO]: Scramble is: ");
	    println!("{scramble}");
	    print!("[INFO]: (Uncompressed: [ "); for m in &scramble.0 { print!("{m} "); } println!("])");

	    println!("[INFO]: Solving...");
	    println!("Scramble to solve:\n{cube}");
	    let r = solve(cube);
	    for m in &r.0 { cube.make_move(m) }
	    println!("Final state:\n{cube}");

	    println!("[RESULT]: Final solution is: {r}");
	    print!("[INFO]: Uncompressed solution: [ "); for m in &r.0 { print!("{m} "); } println!("]");

	    println!();
	    
	    println!("[RESULT]: Reverse of solution: {}", r.reversed());
	    print!("[INFO]: Uncompressed reverse: [ "); for m in r.0.iter().rev() { print!("{} ", m.opposite()); } println!("]");
	},
	"gen" => {
	    println!("[INFO]: Generating `{INPUT_FILE_NAME}`...");
	    write_blank_slate().unwrap();
	    println!("[INFO]: `{INPUT_FILE_NAME}` has been generated, exiting");
	    exit(0);
	},
	"solve" => {
	    println!("[INFO]: Reading from `{INPUT_FILE_NAME}`...");
	    let cube: Cube = match read_from_input_file() {
		Ok(c) => c,
		Err(e) => {
		    println!("[ERROR]: Could not parse `{INPUT_FILE_NAME}`:'{e}'. Please double check `{INPUT_FILE_NAME}`");
		    exit(2);
		},
	    };
	    println!("[INFO]: `{INPUT_FILE_NAME}` has been read");
	    println!("[INFO]: Interpreted cube is:\n{cube}");
	    println!("[INFO]: Starting the solve...");
	    let r = solve(cube);

	    println!("[INFO]: Checking correctness...");
	    let mut checking_cube = cube;
	    for m in &r.0 { checking_cube.make_move(&m) }

	    println!("Starting cube:\n{cube}\n");
	    println!("Final cube:\n{checking_cube}");

	    println!("[RESULT]: Final solution is: {r}");
	    print!("[INFO]: Uncompressed solution: [ "); for m in &r.0 { print!("{m} "); } println!("]");

	    println!();
	    
	    println!("[RESULT]: Reverse of solution: {}", MoveSeq(r.0.clone().into_iter().rev().collect()));
	    print!("[INFO]: Uncompressed reverse: [ "); for m in r.0.iter().rev() { print!("{} ", m.opposite()); } println!("]");
	}
	o => {
	    println!("[ERROR]: option `{o} not recognized. Please use `gen` or `solve`");
	}
    }
}
