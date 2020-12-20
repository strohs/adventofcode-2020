// Day 2: Password Philosophy
// https://adventofcode.com/2020/day/2

use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    chr: char,
    pwd: String,
}

// regex used to parse a line of input
const POLICY_PAT: &'static str = r"(?P<min>\d+)-(?P<max>\d+) (?P<chr>\w): (?P<pwd>\w+)";

/// returns an iterator over the lines of the file pointed to by filename
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// parses the input string, i,e:`12-13 n: nwnwdplnhfhlnnnntfn`, into a Policy struct
fn parse_line(s: &str) -> Policy {
    let re = Regex::new(POLICY_PAT).expect("Pattern should be valid RegEx");
    let caps = re.captures(s).expect("input should match our pattern");
    let min = caps["min"]
        .parse::<usize>()
        .expect("min should be an integer");
    let max = caps["max"]
        .parse::<usize>()
        .expect("max should be an integer");
    let chr = caps["chr"].chars().nth(0).unwrap();
    let pwd = String::from(&caps["pwd"]);

    Policy { min, max, chr, pwd }
}

#[allow(dead_code)]
fn valid_policy(p: &Policy) -> bool {
    let count = p.pwd.chars().filter(|c| *c == p.chr).count();
    count >= p.min && count <= p.max
}

#[allow(dead_code)]
fn valid_policy_position(p: &Policy) -> bool {
    let c1 = p.pwd.chars().nth(p.min - 1).unwrap();
    let c2 = p.pwd.chars().nth(p.max - 1).unwrap();
    (p.chr == c1) ^ (p.chr == c2)
}

fn main() {
    // read a line of input
    let lines = read_lines("../input/02-input.txt").expect("input file to be found");
    let valid_count = lines
        .filter(|line| {
            let l = line.as_ref().unwrap();
            let p = parse_line(l);
            valid_policy_position(&p)
        })
        .count();

    println!("total valid policies {}", valid_count);
}
