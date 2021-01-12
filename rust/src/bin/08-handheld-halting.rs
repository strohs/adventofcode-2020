// Day 8 - Handheld Halting
// https://adventofcode.com/2020/day/8

use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug, Copy, Clone)]
/// All the possible Instructions
enum Ins {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

impl Ins {
    fn from(ins: &str, amount: i32) -> Self {
        match ins {
            "acc" => Ins::ACC(amount),
            "jmp" => Ins::JMP(amount),
            _ => Ins::NOP(amount),
        }
    }

    /// swaps a NOP to a JMP and a JMP to a NOP. ACC returns itself
    #[allow(dead_code)]
    fn swap(ins: &Ins) -> Self {
        match ins {
            Ins::NOP(amt) => Ins::JMP(*amt),
            Ins::JMP(amt) => Ins::NOP(*amt),
            Ins::ACC(_amt) => *ins,
        }
    }

    /// returns true if `ins` is an ACC instruction
    fn is_acc(ins: &Ins) -> bool {
        match ins {
            Ins::ACC(_) => true,
            _ => false,
        }
    }
}

/// parse input file into a Vector of `Ins`tructions
fn parse_input(filename: &str) -> Vec<Ins> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<ins>\w+) (?P<sign>[+-])(?P<amount>\d+)").expect("valid RegEx");
    }
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let caps = RE.captures(&line).expect("input should match RE pattern");
            let ins = &caps["ins"];
            let sign = &caps["sign"];
            let mut amount = caps["amount"].parse::<i32>().unwrap();
            if sign == "-" {
                amount *= -1;
            }
            Ins::from(ins, amount)
        })
        .collect::<Vec<Ins>>()
}

// if the program terminates, returns: (true, final_value_of_accumulator, Vec_of_ins_indices_run)
// if the program does NOT terminate (i.e. it loops), returns:
//      (false, value_of_accumulator_before_executing_loop_ins, Vec_of_ins_run)
fn will_terminate(ins: &Vec<Ins>) -> (bool, i32, Vec<usize>) {
    let mut acc = 0;
    let mut cidx = 0;
    // is a list of indices into `ins` of only NOP and JMP commands
    let mut ins_hist: Vec<usize> = Vec::with_capacity(ins.len());
    let mut visited = vec![false; ins.len()];
    loop {
        if cidx >= ins.len() {
            // program does terminate
            return (true, acc, ins_hist);
        }
        if visited[cidx] == true {
            // program loops
            return (false, acc, ins_hist);
        }

        visited[cidx] = true;
        if !Ins::is_acc(&ins[cidx]) {
            ins_hist.push(cidx);
        }
        match ins[cidx] {
            Ins::ACC(amt) => {
                acc += amt;
                cidx += 1;
            }
            Ins::JMP(amt) => {
                cidx = (cidx as i32 + amt) as usize;
            }
            Ins::NOP(_amt) => {
                cidx += 1;
            }
        }
    }
}

// Immediately before any instruction is executed a second time, what value is in the accumulator?
fn part_one() {
    //
    let ins = parse_input("../input/08-input.txt");
    let (term, acc, _) = will_terminate(&ins);
    if !term {
        println!("program loops with accumulator = {}", &acc);
    } else {
        println!("program terminates with acc = {}", &acc);
    }
}

fn main() {
    part_one();
}
