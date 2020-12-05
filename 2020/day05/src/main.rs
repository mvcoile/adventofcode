
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("input")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    let _vinput: Vec<&str> = input.split_whitespace().collect();

    let mut result: Vec<Boardingpass> = Vec::new();
    for &c in _vinput.iter() {
        let boarding = Boardingpass::new( String::from(c) );
        result.push(boarding);
    }

    result.sort_unstable();

    for b in result.iter() {
        b.println();
    }

    println!("Highest Seat: {:?}", result.last().expect("Failed to get last Boardingpass.").seat);

    let mut prev: u16 = result.first().expect("Failed to get first Boardingpass.").seat-1;
    let mut my_seat: u16 = 0;

    for b in result.iter() {
        if b.seat-1 == prev {
            prev = b.seat;
            continue;
        } else {
            my_seat = (b.seat+prev)/2;
            break;
        }
    }
    println!("My Seat: {:?}", my_seat);

    Ok(())
}

#[derive(Debug)]
#[derive(Eq)]
struct Boardingpass {
    row: u8,
    column: u8,
    seat: u16,
    code: String,
}

#[warn(non_snake_case)]
impl Boardingpass {
    // New Boardingpass for a given code
    fn new(code: String) -> Self {
        let mut rc: u8 = 0;
        let mut rl: u8 = 0;
        let mut ru: u8 = 127;
        let mut cc: u8 = 0;
        let mut cl: u8 = 0;
        let mut cu: u8 = 7;

        // take care to round up/down correctly.
        for c in code.chars() {
            match c {
                'F' => { ru = (rl + ru - ((rl + ru) % 2) ) / 2; rc = ru; },
                'B' => { rl = (rl + ru + ((rl + ru) % 2) ) / 2; rc = rl; },
                'L' => { cu = (cl + cu - ((cl + cu) % 2) ) / 2; cc = cu; },
                'R' => { cl = (cl + cu + ((cl + cu) % 2) ) / 2; cc = cl; },
                _ => ()
            }
        }

        Boardingpass {row: rc, column:cc, seat: (u16::from(rc) * 8 + u16::from(cc)), code: code}
    }

    fn println (&self) {
        println!("{:}: row {:?}, column {:?}, seat ID {:?}", self.code, self.row, self.column, self.seat);
    }

}

impl Ord for Boardingpass {
    fn cmp(&self, other: &Self) -> Ordering {
        self.seat.cmp(&other.seat)
    }
}

impl PartialOrd for Boardingpass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Boardingpass {
    fn eq(&self, other: &Self) -> bool {
        self.seat == other.seat
    }
}
