use std::error::Error;
use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("input")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    let v_input: Vec<&str> = input.split_whitespace().collect();

    let mut expense_report: Vec<u32> = Vec::new();
    for expense_string in v_input.iter() {
        let reg_number = Regex::new(r"\d+").expect("Not a valif Regex.");
        if reg_number.is_match(expense_string) {
            let excpence: u32 = expense_string.parse().expect("String is not a number.");
            expense_report.push(excpence);
        }
    }
    println!("Total number of expenses: {}", expense_report.len());

    expense_report.sort();

    let target: u32 = 2020;
    let mut n = 0;

    let mut result1: u32 = 0;
    for r in &expense_report {
        //println!("r = {}", r);
        for c in expense_report.split_at(n+1).1 {
            //println!("c = {}", c);
            if (r+c)==target {
                result1 = r*c;

                break;
            }
        }
        if result1 > 0 { break };
        n += 1;
    }

    let mut result2: u32 = 0;
    for r in &expense_report {
        for c1 in expense_report.split_at(n+1).1 {
            for c2 in expense_report.split_at(n+2).1 {
                if (r+c1+c2)==target {
                    result2 = r*c1*c2;
                }
                if result2 > 0 { break };
            }
            if result2 > 0 { break };
        }
        if result2 > 0 { break };
        n += 1;
    }

    println!("Expence (part 1) = {}", result1);
    println!("Expence (part 2) = {}", result2);
    println!("Total number of expenses: {}", expense_report.len());

    Ok(())
}
