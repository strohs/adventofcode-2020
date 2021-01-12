use num_traits::PrimInt;
use std::collections::HashMap;
/// Advent of Code - Day 9 - Encoding Error
/// https://adventofcode.com/2020/day/9
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::BufRead;

/// parse input file into a Vector of integers
fn parse_input(filename: &str) -> Vec<i64> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

/// return all pairs of integers from the `v` that sum up to `sum`
/// v - the vector of integers
/// sum - the target sum
fn two_sum<T>(v: &[T], sum: T) -> Vec<(T, T)>
where
    T: PrimInt + Hash,
{
    let mut imap = HashMap::new();
    let mut res = Vec::new();
    for i in v {
        let target = sum - *i;
        if imap.contains_key(&target) {
            res.push((*i, target));
        }
        imap.entry(*i).or_insert(*i);
    }
    res
}

#[allow(dead_code)]
fn valid_pair<T: PrimInt>(pair: &(T, T)) -> bool {
    let (p1, p2) = pair;
    *p1 != *p2
}

#[allow(dead_code)]
fn part_one(nums: Vec<i64>) -> Option<i64> {
    for n in 25..nums.len() {
        let sum_pairs = two_sum(&nums[(n - 25)..n], nums[n]);
        let valid_pairs: Vec<&(i64, i64)> =
            sum_pairs.iter().filter(|&pair| valid_pair(pair)).collect();
        if valid_pairs.is_empty() {
            println!(
                "{} does not have two previous numbers that sum to it",
                &nums[n]
            );
            return Some(nums[n]);
        }
    }
    None
}

// part 2 functions start here

/// returns a vector of sorted integers, that are a contiguous slice from `nums` that
/// sum up to `target`
/// This is gonna use a brute force approach
fn part_two(nums: Vec<i64>, target: i64) -> Option<Vec<i64>> {
    for window_size in 2..nums.len() {
        for window in nums.windows(window_size) {
            let mut slice = window.to_owned();
            slice.sort_unstable();
            // don't bother summing a slice that contains an element >= target
            if *slice.last().unwrap() < target {
                if slice.iter().sum::<i64>() == target {
                    let enc_weak = slice.first().unwrap() + slice.last().unwrap();
                    println!(
                        "encryption weakness = {} from elements {:?}",
                        &enc_weak, &slice
                    );
                    return Some(slice);
                }
            }
        }
    }
    None
}

fn main() {
    let nums = parse_input("../input/09-input.txt");
    //part_one(nums);
    part_two(nums, 41682220);
}
