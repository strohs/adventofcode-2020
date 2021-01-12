// Day 8 - Handheld Halting - Part 2
// https://adventofcode.com/2020/day/8

use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;

/// Ins represents all the possible Instructions for this challenge
#[derive(Debug, Copy, Clone)]
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

/// returns a Vector of indices, of NOP and JMP statements that were executed before a loop occurred
fn ins_indices(ins: &Vec<Ins>) -> Option<Vec<usize>> {
    let mut cidx = 0;
    // is a list of indices into `ins` of only NOP and JMP commands
    let mut ins_hist: Vec<usize> = vec![];
    let mut visited = vec![false; ins.len()];
    loop {
        if cidx >= ins.len() {
            // program terminated
            return None;
        }
        if visited[cidx] == true {
            // program loops
            return Some(ins_hist);
        }

        visited[cidx] = true;
        if !Ins::is_acc(&ins[cidx]) {
            ins_hist.push(cidx);
        }
        match ins[cidx] {
            Ins::ACC(_amt) => {
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

/// if the program given by `ins` terminates, `Some(i32)` is returned containing the final
/// accumulator value. If the program doesn't terminate, `None` is returned
fn terminates(ins: &Vec<Ins>) -> Option<i32> {
    let mut acc = 0;
    let mut cidx = 0;
    let mut visited = vec![false; ins.len()];
    loop {
        if cidx >= ins.len() {
            // program terminates
            return Some(acc);
        }
        if visited[cidx] == true {
            // program has a loop
            return None;
        }

        visited[cidx] = true;
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

fn part_two() {
    let mut ins = parse_input("../input/08-input.txt");

    if let Some(mut ins_hist) = ins_indices(&ins) {
        let mut last_idx = *ins_hist.last().unwrap();
        let mut last_ins = ins[last_idx];

        loop {
            if let Some(acc) = terminates(&ins) {
                println!("final acc = {}", &acc);
                break;
            }
            // else we need to: restore the last instruction that was swapped
            ins[last_idx] = last_ins;

            // swap the last JMP or NOP instruction and try again
            if let Some(ih_idx) = ins_hist.pop() {
                last_idx = ih_idx;
                last_ins = ins[ih_idx];
                ins[ih_idx] = Ins::swap(&ins[ih_idx]);
            } else {
                panic!("no JMP or NOP statements found to change, but we still have loops")
            }
        }
    } else {
        panic!("no JMP or NOP statements found to change")
    }
}

fn main() {
    part_two();
}
