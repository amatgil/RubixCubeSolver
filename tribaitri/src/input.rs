use std::{borrow::Cow, error::Error, fs::File, io::Write, process::exit};

use shared::{Color, Solvable};

use crate::Cube3;


pub fn write_three_file() -> Result<(), Box<dyn std::error::Error>>{
    let template = 
"    ┏━━━┓
    ┃XXX┃
    ┃XXX┃
    ┃XXX┃
┏━━━╋━━━╋━━━┳━━━┓
┃XXX┃XXX┃XXX┃XXX┃
┃XXX┃XXX┃XXX┃XXX┃
┃XXX┃XXX┃XXX┃XXX┃
┗━━━╋━━━╋━━━┻━━━┛
    ┃XXX┃
    ┃XXX┃
    ┃XXX┃
    ┗━━━┛";

    let mut file = File::create(Cube3::INPUT_FILE_NAME)?;
    file.write(template.as_bytes())?;

    Ok(())
}

fn get_next_color(input: &mut impl Iterator<Item = char>, error_s: String) -> Result<Color, Box<dyn Error>> {
    let c = input.next().ok_or(error_s.clone())?;

    let col = Color::from(c);
    col.ok_or(format!("{c} is not a valid color").into())
}

fn skip_n_chars(input: &mut impl Iterator<Item = char>, n: usize, e: String) -> Result<(), String>{
    for _ in 0..n { input.next().ok_or(e.clone())?; }
    Ok(())
}

pub fn read_three_from_string(input: &str) -> Result<Cube3, Box<dyn Error>> {
    let error_s: Cow<str> = format!("File {} does not represent a cube (valid or non-valid)", Cube3::INPUT_FILE_NAME).into();
    //let mut s = Stickers::default();
    println!("[ERROR]: Not yet implemented!");
    exit(4);
    let mut input = input.chars();


    todo!()
}
