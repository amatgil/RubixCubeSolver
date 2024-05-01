use crate::Solvable;


pub fn generate_cube_file<C: Solvable>() -> C {
    println!("[INFO]: Generating `{}`...", C::INPUT_FILE_NAME);
    C::write_blank_slate().unwrap();
    println!("[INFO]: `{}` has been generated, exiting", C::INPUT_FILE_NAME);
    std::process::exit(0)
}
