use std::{env, process::exit};

use tubaitu::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(option) = args.get(1) else {
	println!("[ERROR]: Missing argument: `gen` to (re-)generate template file or `solve` to read from it and output solving steps");
	exit(1);
    };

    match &**option {
	"gen" => {
	    println!("[INFO]: Generating `{INPUT_FILE_NAME}`...");
	    write_blank_slate().unwrap();
	    println!("[INFO]: `{INPUT_FILE_NAME}` has been generated, exiting");
	    exit(0);
	},
	"solve" => {
	    println!("[INFO]: Reading from `{INPUT_FILE_NAME}`...");
	    let cube = read_from_input_file().unwrap();
	    println!("[INFO]: `{INPUT_FILE_NAME}` has been read");
	    println!("[INFO]: Interpreted cube is:\n{cube}");
	    println!("[INFO]: Starting the solve...");
	    let r = solve(cube);

	    println!("[INFO]: Checking correctness...");
	    let mut checking_cube = cube;
	    for m in &r { checking_cube.make_move(&m) }

	    println!("Starting cube:\n{cube}\n");
	    println!("Final cube:\n{checking_cube}");
	    print_solution(&r);
	    print_reverse_solution(&r);
	}
	o => {
	    println!("[ERROR]: option `{o} not recognized. Please use `gen` or `solve`");
	}
    }
}
