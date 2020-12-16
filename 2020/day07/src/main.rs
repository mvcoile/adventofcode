use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("input")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    let v_input: Vec<&str> = input.trim().lines().collect();

    println!("The input has {} lines", v_input.len());


    Ok(())
}
