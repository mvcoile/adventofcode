use std::error::Error;
use std::fs::File;
use std::io::Read;
//use std::cmp::Ordering;

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = File::open("input")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    let v_input: Vec<&str> = input.split("\n\n").collect();

    let mut passports: Vec<Passport> = Vec::new();
    for &p in v_input.iter() {
        let passport = Passport::parse( &p.to_string() );
        //passport.println();
        passports.push(passport);
    }

    let mut valid_part1: u8 = 0;
    for p in passports.iter() {

        if p.is_valid_part1() {
            valid_part1 = valid_part1 + 1;
        }
    }

    let mut valid_part2: u8 = 0;
    for p in passports.iter() {

        if p.is_valid() {
            valid_part2 = valid_part2 + 1;
        }
    }

    println!("Total number of Passports: {}", v_input.len());
    println!("Valid (1) number of Passports: {}",  valid_part1);
    println!("Valid (2) number of Passports: {}",  valid_part2);

    Ok(())
}

#[derive(Debug)]
struct Passport {
    byr: String, // (Birth Year)
    iyr: String, // (Issue Year)
    eyr: String, // (Expiration Year)
    hgt: String, // (Height)
    hcl: String, // (Hair Color)
    ecl: String, // (Eye Color)
    pid: String, // (Passport ID)
    cid: String, // (Country ID)
}

impl Passport {
    fn new(byr: String, iyr: String, eyr: String, hgt: String, hcl: String, ecl: String, pid: String, cid: String) -> Self { Self { byr, iyr, eyr, hgt, hcl, ecl, pid, cid } }

    fn parse(data: &String) -> Self {
        let fields_vec: Vec<&str> = data.split_whitespace().collect();

        let mut _byr: String = String::new();
        let mut _iyr: String = String::new();
        let mut _eyr: String = String::new();
        let mut _hgt: String = String::new();
        let mut _hcl: String = String::new();
        let mut _ecl: String = String::new();
        let mut _pid: String = String::new();
        let mut _cid: String = String::new();

        for field in fields_vec {
            let field = String::from(field);
            let keyvalue: Vec<&str> = field.split(':').collect();
            let key = keyvalue.first().expect("Key not found.");
            let value = keyvalue.last().expect("Value not found.");

            match key {
                &"byr" => {_byr = value.to_string()},
                &"iyr" => {_iyr = value.to_string()},
                &"eyr" => {_eyr = value.to_string()},
                &"hgt" => {_hgt = value.to_string()},
                &"hcl" => {_hcl = value.to_string()},
                &"ecl" => {_ecl = value.to_string()},
                &"pid" => {_pid = value.to_string()},
                &"cid" => {_cid = value.to_string()},
                _ => ()
            }
        }

        Passport::new(_byr, _iyr, _eyr, _hgt, _hcl, _ecl, _pid, _cid)
    }

    // Pretty println
    fn println (&self) {
        println!("Passport ({})
    Birth Year      : {}
    Issue Year      : {}
    Expiration Year : {}
    Height          : {}
    Hair Color      : {}
    Eye Color       : {}
    Passport ID     : {}
    Country ID      : {}
    Valid           : {}",
            &self.pid, &self.byr, &self.iyr, &self.eyr, &self.hgt, &self.hcl, &self.ecl, &self.pid, &self.cid, &self.is_valid()
        );
    }

    fn is_valid(&self) -> bool {

        self.is_byr_valid() &&
        self.is_iyr_valid() &&
        self.is_eyr_valid() &&
        self.is_hgt_valid() &&
        self.is_hcl_valid() &&
        self.is_ecl_valid() &&
        self.is_pid_valid() &&
        self.is_cid_valid()
    }

    fn is_valid_part1(&self) -> bool {
        !self.byr.is_empty() &&
        !self.iyr.is_empty() &&
        !self.eyr.is_empty() &&
        !self.hgt.is_empty() &&
        !self.hcl.is_empty() &&
        !self.ecl.is_empty() &&
        !self.pid.is_empty()
    }

    fn is_byr_valid(&self) -> bool {
        match self.byr.parse::<u16>() {
            Ok(u) => (u >= 1920 && u <= 2002),
            Err(_error) => false
        }
    }

    fn is_iyr_valid(&self) -> bool {
        match self.iyr.parse::<u16>() {
            Ok(u) => (u >= 2010 && u <= 2020),
            Err(_error) => false
        }
    }

    fn is_eyr_valid(&self) -> bool {
        match self.eyr.parse::<u16>() {
            Ok(u) => (u >= 2020 && u <= 2030),
            Err(_error) => false
        }
    }

    fn is_hgt_valid(&self) -> bool {
        let cm_reg = Regex::new(r"^(\d{3}cm)$").expect("Malformed Regex.");
        let in_reg = Regex::new(r"^(\d{2}in)$").expect("Malformed Regex.");

        if cm_reg.is_match(self.hgt.as_str()) {
            let reg = Regex::new(r"(\d{3})").expect("Malformed Regex.");
            let temp = reg.captures(self.hgt.as_str()).unwrap().get(1).unwrap().as_str();

            match temp.parse::<u16>() {
                Ok(u) => (u >= 150 && u <= 193),
                Err(_error) => false
            }
        } else if in_reg.is_match(self.hgt.as_str()) {
            let reg = Regex::new(r"(\d{2})").expect("Malformed Regex.");
            let temp = reg.captures(self.hgt.as_str()).unwrap().get(1).unwrap().as_str();

            match temp.parse::<u16>() {
                Ok(u) => (u >= 59 && u <= 76),
                Err(_error) => false
            }
        } else {
            false
        }
    }

    fn is_hcl_valid(&self) -> bool {
        let reg = Regex::new(r"^(\#[0-9a-fA-F]{6})$").expect("Malformed Regex.");
        reg.is_match(self.hcl.as_str())
    }

    fn is_ecl_valid(&self) -> bool {
        let reg = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").expect("Malformed Regex.");
        reg.is_match(self.ecl.as_str())
    }

    fn is_pid_valid(&self) -> bool {
        let reg = Regex::new(r"^(\d{9})$").expect("Malformed Regex.");
        reg.is_match(self.pid.as_str())
    }

    fn is_cid_valid(&self) -> bool {
        true
    }

}
