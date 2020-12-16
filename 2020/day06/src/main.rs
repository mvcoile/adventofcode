use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("input")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    let v_input: Vec<&str> = input.trim().split("\n\n").collect();

    println!("The input has {} groups", v_input.len());

    let mut part1: usize = 0;
    for v in &v_input {
        let mut chars: Vec<char> = v.chars().collect::<Vec<char>>();
        chars.retain(|c| c != &'\n');
        chars.sort();
        chars.dedup();
        //println!("{:?}", chars);
        part1 += chars.len();
    }
    println!("What is the sum of those counts? {} (Part 1)", part1);

    let mut part2: usize = 0;
    for g in &v_input {
        let lines: Vec<&str> = g.lines().collect();

        let mut result: Vec<char> = lines.first().unwrap().chars().collect::<Vec<char>>();
        result.sort();
        result.dedup();

        for l in lines {
            //print!("{:?}", l);
            //println!(" ({:?})", result);
            for c in result.clone() {
                if !l.contains(&c.to_string()) {
                    result.retain(|r| r != &c );
                }
            }
        }

        result.sort();
        result.dedup();
        part2 += result.len();
    }
    println!("What is the sum of those counts? {} (Part 2)", part2);

    Ok(())
}
