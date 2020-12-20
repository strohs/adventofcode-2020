use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

/// Day 1 - Advent of Code
/// https://adventofcode.com/2020/day/1

fn read_lines<P>(filename: P) -> Vec<i32>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("input should be in input/01-input.txt");
    let reader = io::BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    lines
}

/// return all triplets of integers that add up to `target_sum`
fn three_sum(ls: &[i32], target_sum: i32) -> Vec<(i32, i32, i32)> {
    let mut triples: Vec<(i32, i32, i32)> = vec![];

    for i in 0..(ls.len() - 1) {
        let target = target_sum - ls[i];
        let pairs = two_sum(&ls[i + 1..], target);
        for pair in pairs {
            triples.push((ls[i], pair.0, pair.1));
        }
    }
    triples
}

/// find two entries in ls that sum to the `target_sum`. ls must be a sorted list
fn two_sum(ls: &[i32], target_sum: i32) -> Vec<(i32, i32)> {
    let mut results: Vec<(i32, i32)> = vec![];
    for i in 0..(ls.len() - 1) {
        let target = target_sum - ls[i];
        let sub = &ls[i + 1..];
        match sub.binary_search(&target) {
            Ok(n) => results.push((ls[i], sub[n])),
            _ => {}
        }
    }
    results
}

// Find two entries 01-input.txt that sum to 2020
fn main() {
    // read sample data from file and sort it
    let mut entries = read_lines("./input/01-input.txt");
    entries.sort();

    // find triplets that sum to 2020
    let results = three_sum(&entries, 2020);

    //let product = results.iter().fold(1, |acc, n| acc * n.0 * n.1);
    let product = results.iter().fold(1, |acc, n| acc * n.0 * n.1 * n.2);
    println!("the numbers are {:?}", &results);
    println!("with a product of {}", &product);
}
