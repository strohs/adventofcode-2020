// Day 5: Binary Boarding - Part 2
// https://adventofcode.com/2020/day/5

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

/// returns an iterator over the lines of the file pointed to by filename
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn compute_row(s: &str) -> u32 {
    s.chars()
        .fold((0, 127), |(lo, hi), c| {
            let mid = (lo + hi) / 2;
            match c {
                'F' => (lo, mid),
                'B' => (mid + 1, hi),
                _ => panic!("unknown row character"),
            }
        })
        .0
}

fn compute_col(s: &str) -> u32 {
    s.chars()
        .fold((0, 7), |(lo, hi), c| {
            let mid = (lo + hi) / 2;
            match c {
                'L' => (lo, mid),
                'R' => (mid + 1, hi),
                _ => panic!("unknown col character"),
            }
        })
        .0
}

fn seat_id(row: u32, col: u32) -> u32 {
    row * 8 + col
}

fn main() {
    let mut seats: Vec<u32> = read_lines("../input/05-input.txt")
        .unwrap()
        .map(|lines| {
            let line = lines.as_ref().unwrap();
            let col_start_idx = line
                .find(|c: char| c == 'R' || c == 'L')
                .expect("must have 'R' or 'L' char in string");
            let row = compute_row(&line[0..col_start_idx]);
            let col = compute_col(&line[col_start_idx..]);
            seat_id(row, col)
        })
        .collect();

    seats.sort();
    for i in 0..(seats.len() - 1) {
        if seats[i] + 1 != seats[i + 1] {
            println!("missing seat {}", &seats[i] + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::compute_col;
    use super::compute_row;
    use crate::seat_id;

    #[test]
    fn compute_row_test() {
        assert_eq!(compute_row("FBFBBF"), 44);
    }

    #[test]
    fn compute_col_test() {
        assert_eq!(compute_col("RLR"), 5);
    }

    #[test]
    fn compute_col_test_all_r() {
        assert_eq!(compute_col("RRR"), 7);
    }

    #[test]
    fn compute_col_test_all_l() {
        assert_eq!(compute_col("LLL"), 0);
    }

    #[test]
    fn seat_id_test() {
        assert_eq!(seat_id(44, 5), 357);
    }
}
