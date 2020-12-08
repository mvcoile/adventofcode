use std::error::Error;
use std::fs::File;
use std::io::Read;
//use std::cmp::Ordering;

//use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("input")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    let v_input: Vec<&str> = input.trim().split('\n').collect();

    let mut valid1 = 0;
    for &p in v_input.iter() {
        if !p.is_empty() {
            let password = Password::parse( p.to_string() );
            if password.is_valid1() {
                //println!("{:?}", password);
                valid1 += 1;
            }
        }
    }

    let mut valid2 = 0;
    for &p in v_input.iter() {
        if !p.is_empty() {
            let password = Password::parse( p.to_string() );
            if password.is_valid2() {
                //println!("{:?}", password);
                valid2 += 1;
            }
        }
    }

    println!("Total number valid (Part 1) of Passwords: {}", valid1);
    println!("Total number valid (Part 2) of Passwords: {}", valid2);

    Ok(())
}

#[derive(Debug)]
struct Password {
    password: String,
    pattern: char,
    lower: usize,
    upper: usize,
}

impl Password {
    /*    Example Data
     *    5-6 r: rrrmrr
     *    1-2 k: lkbhbkstth
     *    17-18 j: jjjjjjjjjjzjjjjjxsj
     *    4-12 t: tttkttttttttttttj
     *    7-9 k: kkkkkkwkk
     *    3-5 q: pjlql
     */
    fn new(password: String, pattern: char, lower: usize, upper: usize) -> Self { Self { password, pattern, lower, upper } }

    fn parse(data: String) -> Self {
        let temp: Vec<&str> = data.splitn(2, '-').collect();
        //println!("{:?}", temp);
        let left:  String = temp[0].to_string();
        let right: String = temp[1].to_string();

        let low: usize = left.parse().unwrap();

        let temp: Vec<&str> = right.splitn(2, ' ').collect();
        let left:  String = temp[0].to_string();
        let right: String = temp[1].to_string();

        let up: usize = left.parse().unwrap();

        let temp: Vec<&str> = right.splitn(2, ": ").collect();
        let left:  String = temp[0].to_string();
        let right: String = temp[1].to_string();

        let pat: char = left.parse().unwrap();
        let pass: String = right.parse().unwrap();

        Password::new(pass, pat, low, up)
    }

    fn is_valid1(&self) -> bool {
        let matches: Vec<&str> = self.password.matches(self.pattern).collect();

        matches.len() >= self.lower && matches.len() <= self.upper
    }

    fn is_valid2(&self) -> bool {
        if self.password.is_char_boundary(self.lower) && self.password.is_char_boundary(self.upper) {
            let mut up: bool = false;
            let mut lo: bool = false;
            for mi in self.password.match_indices(self.pattern) {
               let (index, _string) = mi;
               if index+1 == self.upper {
                   up = true;
               } else if index+1 == self.lower {
                   lo = true;
               }
            }
            up ^ lo
        } else { false }
    }
}
