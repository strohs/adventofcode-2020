// Day 4: Passport Processing
// https://adventofcode.com/2020/day/4

use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Debug)]
pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl<'a> Passport {
    fn new() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn byr(&mut self, byr: Option<String>) -> &mut Passport {
        self.byr = byr;
        self
    }

    fn valid_byr(&self) -> bool {
        if self.byr.is_some() {
            let year = self.byr.as_ref().unwrap().parse::<i32>().unwrap();
            year >= 1920 && year <= 2002
        } else {
            false
        }
    }

    fn iyr(&mut self, iyr: Option<String>) -> &mut Passport {
        self.iyr = iyr;
        self
    }

    fn valid_iyr(&self) -> bool {
        if self.iyr.is_some() {
            let year: i32 = self.iyr.as_ref().unwrap().parse::<i32>().unwrap();
            year >= 2010 && year <= 2020
        } else {
            false
        }
    }

    fn eyr(&mut self, eyr: Option<String>) -> &mut Passport {
        self.eyr = eyr;
        self
    }

    fn valid_eyr(&self) -> bool {
        if self.eyr.is_some() {
            let year: i32 = self.eyr.as_ref().unwrap().parse::<i32>().unwrap();
            year >= 2020 && year <= 2030
        } else {
            false
        }
    }

    fn hgt(&mut self, hgt: Option<String>) -> &mut Passport {
        self.hgt = hgt;
        self
    }

    fn valid_hgt(&self) -> bool {
        if self.hgt.is_some() {
            let hgt = self.hgt.as_ref().unwrap();
            if let Some(pos) = hgt.find("cm") {
                let cm: i32 = hgt[0..pos].parse().expect("cm value to be digits");
                if cm >= 150 && cm <= 193 {
                    true
                } else {
                    false
                }
            } else if let Some(pos) = hgt.find("in") {
                let inches: i32 = hgt[0..pos].parse().expect("inches value to be digits");
                if inches >= 59 && inches <= 76 {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn hcl(&mut self, hcl: Option<String>) -> &mut Passport {
        self.hcl = hcl;
        self
    }

    fn valid_hcl(&self) -> bool {
        self.hcl.is_some()
    }

    fn ecl(&mut self, ecl: Option<String>) -> &mut Passport {
        self.ecl = ecl;
        self
    }

    fn valid_ecl(&self) -> bool {
        self.ecl.is_some()
    }

    fn pid(&mut self, pid: Option<String>) -> &mut Passport {
        self.pid = pid;
        self
    }

    fn valid_pid(&self) -> bool {
        self.pid.is_some()
    }

    fn cid(&mut self, cid: Option<String>) -> &mut Passport {
        self.cid = cid;
        self
    }

    /// passport is valid if all required fields are present, AND valid
    /// cid field is ignored in this scenario
    fn is_valid(&self) -> bool {
        self.valid_byr()
            && self.valid_ecl()
            && self.valid_eyr()
            && self.valid_hcl()
            && self.valid_hgt()
            && self.valid_iyr()
            && self.valid_pid()
    }
}

/// returns an iterator over the lines of the file pointed to by filename
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_byr_line(line: &String, pp: &mut Passport) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"byr:(\d\d\d\d)\b").expect("Pattern should be valid RegEx");
    }
    if let Some(cap) = RE.captures(line) {
        pp.byr(cap.get(1).map(|m| String::from(m.as_str())));
    }
}

fn parse_iyr_line(line: &String, pp: &mut Passport) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"iyr:(\d\d\d\d)\b").expect("Pattern should be valid RegEx");
    }
    if let Some(cap) = RE.captures(line) {
        pp.iyr(cap.get(1).map(|m| String::from(m.as_str())));
    }
}

fn parse_eyr_line(line: &String, pp: &mut Passport) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"eyr:(\d\d\d\d)\b").expect("Pattern should be valid RegEx");
    }
    if let Some(cap) = RE.captures(line) {
        pp.eyr(cap.get(1).map(|m| String::from(m.as_str())));
    }
}

fn parse_hgt_line(line: &String, pp: &mut Passport) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"hgt:(\d+(cm|in))\b").expect("Pattern should be valid RegEx");
    }
    if let Some(cap) = RE.captures(line) {
        pp.hgt(cap.get(1).map(|m| String::from(m.as_str())));
    }
}

fn parse_hcl_line(line: &String, pp: &mut Passport) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"hcl:(#[a-f0-9]{6})\b").expect("Pattern should be valid RegEx");
    }
    if let Some(cap) = RE.captures(line) {
        pp.hcl(cap.get(1).map(|m| String::from(m.as_str())));
    }
}

fn parse_ecl_line(line: &String, pp: &mut Passport) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b")
            .expect("Pattern should be valid RegEx");
    }
    if let Some(cap) = RE.captures(line) {
        pp.ecl(cap.get(1).map(|m| String::from(m.as_str())));
    }
}

fn parse_pid_line(line: &String, pp: &mut Passport) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"pid:(\d{9})\b").expect("Pattern should be valid RegEx");
    }
    if let Some(cap) = RE.captures(line) {
        pp.pid(cap.get(1).map(|m| String::from(m.as_str())));
    }
}

fn parse_cid_line(line: &String, pp: &mut Passport) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"cid:(#?\w+)").expect("Pattern should be valid RegEx");
    }
    if let Some(cap) = RE.captures(line) {
        pp.cid(cap.get(1).map(|m| String::from(m.as_str())));
    }
}

fn main() {
    let mut valid_count: usize = 0;
    let mut passport = Passport::new();

    for line in read_lines("../input/04-input.txt").unwrap() {
        let s = line.unwrap();
        if !s.is_empty() {
            parse_byr_line(&s, &mut passport);
            parse_iyr_line(&s, &mut passport);
            parse_eyr_line(&s, &mut passport);
            parse_hgt_line(&s, &mut passport);
            parse_hcl_line(&s, &mut passport);
            parse_ecl_line(&s, &mut passport);
            parse_pid_line(&s, &mut passport);
            parse_cid_line(&s, &mut passport);
        } else {
            // hit a blank line, check current passport for validity
            if passport.is_valid() {
                println!("   VALID {:?}", &passport);
                valid_count += 1;
            }
            passport = Passport::new();
        }
    }

    if passport.is_valid() {
        println!("   VALID {:?}", &passport);
        valid_count += 1;
    }

    println!("Total Valid Passports {}", valid_count);
}
