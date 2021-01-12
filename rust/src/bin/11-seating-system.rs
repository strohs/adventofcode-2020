// Advent of Code - Day 11 - Seating System
// https://adventofcode.com/2020/day/11

use std::fmt::{Display, Formatter};
use std::{fmt, io};
use std::convert::TryFrom;
use std::fs::File;
use std::io::BufRead;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl Display for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Seat::Floor => write!(f, "."),
            Seat::Empty => write!(f, "L"),
            Seat::Occupied => write!(f, "#"),
        }
    }
}

impl TryFrom<char> for Seat {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Seat::Floor),
            'L' => Ok(Seat::Empty),
            '#' => Ok(Seat::Occupied),
            _ => Err("invalid seat type char"),
        }
    }
}

#[derive(Debug)]
struct SeatVec {
    seats: Vec<Seat>,
    col_len: usize,
}

impl Display for SeatVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        for (i, s) in self.seats.iter().enumerate() {
            if i > 0 && i % self.col_len == 0 {
                write!(f, "\n")?;
            }
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl SeatVec {
    fn new(seats: Vec<Seat>, col_len: usize) -> Self {
        Self {
            seats,
            col_len,
        }
    }

    fn adjacent_indices(&self, i: usize) -> Vec<usize> {
        let on_top_row = || i < self.col_len;
        let on_bot_row = || i >= self.seats.len() - self.col_len && i < self.seats.len();
        let on_lft_col = || i % self.col_len == 0;
        let on_rgt_col = || i % self.col_len == self.col_len - 1;

        let ul = || i - self.col_len - 1;
        let up = || i - self.col_len;
        let ur = || i - self.col_len + 1;
        let lf = || i - 1;
        let rg = || i + 1;
        let dl = || i + self.col_len - 1;
        let dn = || i + self.col_len;
        let dr = || i + self.col_len + 1;

        let adjs = match i {
            _i if on_top_row() && on_lft_col() => vec![rg(), dn(), dr()],
            _i if on_top_row() && on_rgt_col() => vec![dn(), dl(), lf()],
            _i if on_top_row() => vec![rg(), dr(), dn(), dl(), lf()],
            _i if on_bot_row() && on_lft_col() => vec![up(), ur(), rg()],
            _i if on_bot_row() && on_rgt_col() => vec![lf(), ul(), up()],
            _i if on_bot_row() => vec![up(), ur(), rg(), lf(), ul()],
            _i if on_lft_col() => vec![up(), ur(), rg(), dr(), dn()],
            _i if on_rgt_col() => vec![dn(), dl(), lf(), ul(), up()],
            _ => vec![up(), ur(), rg(), dr(), dn(), dl(), lf(), ul()],
        };
        adjs
    }

    /// returns the index of the first occupied seat in `idxs`. If an empty seat is encountered
    /// first, then None is returned
    fn visible_occupied_seat(&self, idxs: Vec<usize>) -> Option<usize> {
        // dbg!(&idxs, &idxs.len());
        for i in idxs {
            match self.seats.get(i) {
                Some(Seat::Occupied) => return Some(i),
                Some(Seat::Empty) => return None,
                _ => (),
            }
        }
        None
    }

    /// returns the index of the first empty seat that can be seen from index `idx` by looking
    /// vertically up the column
    fn occupied_seat_up(&self, idx: usize) -> Option<usize> {
        let start = idx % self.col_len;
        let end = idx;

        let idxs: Vec<usize> =  (start..end).step_by(self.col_len).rev().collect();
        self.visible_occupied_seat(idxs)
    }

    /// returns the index of the first empty seat that can be seen by looking down the column
    /// that `idx` is on
    fn occupied_seat_down(&self, idx: usize) -> Option<usize> {
        let start = idx + self.col_len;
        let end = (self.seats.len() - self.col_len) + (idx % self.col_len) + self.col_len;

        let idxs: Vec<usize> = (start..end).step_by(self.col_len).collect();
        self.visible_occupied_seat(idxs)
    }

    /// returns index of first occupied seat that can be seen from `idx` looking to the right
    fn occupied_seat_right(&self, idx:usize) -> Option<usize> {
        let start = idx + 1;
        let end = idx + (self.col_len - (idx % self.col_len));

        let idxs: Vec<usize> = (start..end).collect();
        self.visible_occupied_seat(idxs)
    }

    /// returns index of first occupied seat that can be seen from `idx` looking to the left
    fn occupied_seat_left(&self, idx:usize) -> Option<usize> {
        let start = idx - (idx % self.col_len);
        let end = idx;

        let idxs: Vec<usize> = (start..end).rev().collect();
        self.visible_occupied_seat(idxs)
    }

    /// returns index of first occupied seat that can be seen from `idx` looking diagonally up/right
    fn occupied_seat_up_right(&self, idx: usize) -> Option<usize> {
        let on_top_edge = |i: usize| i < self.col_len;
        let on_right_edge = |i: usize| i % self.col_len == self.col_len - 1;

        if on_top_edge(idx) || on_right_edge(idx) {
            None
        } else {
            let mut idxs = vec![];
            let mut idx = idx;
            // while we haven't gone past the top row or right edge of the matrix
            while !on_top_edge(idx) && !on_right_edge(idx)  {
                idx = idx - self.col_len + 1;
                idxs.push(idx);
            }

            self.visible_occupied_seat(idxs)
        }
    }

    /// returns index of first seat that can be seen from `idx` looking diagonally up/left
    fn occupied_seat_up_left(&self, idx: usize) -> Option<usize> {
        let on_top_edge = |i: usize| i < self.col_len;
        let on_left_edge = |i: usize| i % self.col_len == 0;

        if on_top_edge(idx) || on_left_edge(idx) {
            None
        } else {
            let mut idxs = vec![];
            let mut idx = idx;
            // while we haven't gone past the top row or left edge of the seats matrix
            while !on_top_edge(idx) && !on_left_edge(idx)  {
                idx = idx - self.col_len - 1;
                idxs.push(idx);
            }

            self.visible_occupied_seat(idxs)
        }
    }

    /// returns index of first seat that can be seen from `idx` looking diagonally down/left
    fn occupied_seat_down_left(&self, idx: usize) -> Option<usize> {
        let on_bottom_edge = |i: usize| i >= self.seats.len() - self.col_len;
        let on_left_edge = |i: usize| i % self.col_len == 0;

        if on_bottom_edge(idx) || on_left_edge(idx) {
            None
        } else {
            let mut idxs = vec![];
            let mut idx = idx;
            // loop until the top row or left edge of the seats matrix is reached
            loop {
                idx = idx + self.col_len - 1;
                idxs.push(idx);

                if on_bottom_edge(idx) || on_left_edge(idx) {
                    break
                }
            }

            self.visible_occupied_seat(idxs)
        }
    }

    /// returns index of first seat that can be seen from `idx` looking diagonally down/right
    fn occupied_seat_down_right(&self, idx: usize) -> Option<usize> {
        let on_bottom_edge = |i: usize| i >= self.seats.len() - self.col_len;
        let on_right_edge = |i: usize| i % self.col_len == self.col_len - 1;

        if on_bottom_edge(idx) || on_right_edge(idx) {
            None
        } else {
            let mut idxs = vec![];
            let mut idx = idx;
            // loop until the bottom row or right edge of the seats matrix is reached
            loop {
                idx = idx + self.col_len + 1;
                idxs.push(idx);

                if on_bottom_edge(idx) || on_right_edge(idx) {
                    break
                }
            }

            self.visible_occupied_seat(idxs)
        }
    }


    /// returns the total number of seats, (adjacent to the seat at idx), that are occupied
    fn adjacent_occupied_count(&self, idx: usize) -> usize {
        self.adjacent_indices(idx)
            .iter()
            .filter(|&&i| *self.seats.get(i).unwrap() == Seat::Occupied)
            .count()
    }

    /// returns the total number of occupied seats that can be "seen" from the seat at `idx`
    fn visible_occupied_count(&self, idx: usize) -> usize {
        vec![
            self.occupied_seat_left(idx),
            self.occupied_seat_up_left(idx),
            self.occupied_seat_up(idx),
            self.occupied_seat_up_right(idx),
            self.occupied_seat_right(idx),
            self.occupied_seat_down_right(idx),
            self.occupied_seat_down(idx),
            self.occupied_seat_down_left(idx)
        ]
            .iter()
            .filter(|&&vs| vs.is_some())
            .count()
    }

    /// returns total number of seats that are occupied
    fn occupied_seat_count(&self) -> usize {
        self.seats.iter().filter(|s| **s == Seat::Occupied).count()
    }

    // fn adjacents(&mut self, i: usize) -> Vec<&mut Seat> {
    //     let ai = self.adjacent_indices(i);
    //
    //     let ajs: Vec<&mut Seat> = self.seats.borrow_mut()
    //         .iter_mut()
    //         .enumerate()
    //         .filter(|(i, _s)| ai.contains(i))
    //         .map(|(_, s)| s)
    //         .collect();
    //     ajs
    // }
}

/// parse input into a SeatVec Struct
fn parse_input(filename: &str) -> SeatVec {

    let file = File::open(filename).unwrap();
    let mut col_len = 0_usize;

    let mut seats = vec![];
    for (idx, line) in io::BufReader::new(file).lines().enumerate() {
        let line = line.expect("input is valid");
        let mut seat_line: Vec<Seat> = line
            .chars()
            .map(|c| Seat::try_from(c).unwrap())
            .collect();
        if idx == 0 {
            col_len = seat_line.len();
        }
        seats.append(&mut seat_line);
    }

    SeatVec::new(seats, col_len)
}

#[allow(dead_code)]
fn part_one() {
    let mut sv = parse_input("../input/11-input.txt");
    let mut changed = true;
    println!("{}", &sv);
    println!();

    while changed {
        changed = false;
        let mut ts = vec![Seat::Floor; sv.seats.len()];

        for idx in 0..sv.seats.len() {
            match sv.seats.get(idx) {
                Some(Seat::Empty) if sv.adjacent_occupied_count(idx) == 0 => {
                    ts[idx] = Seat::Occupied;
                    changed = true;
                },
                Some(Seat::Occupied) if sv.adjacent_occupied_count(idx) >= 4 => {
                    ts[idx] = Seat::Empty;
                    changed = true;
                },
                _ => ts[idx] = sv.seats[idx],
            }
        }
        if changed {
            sv.seats = ts;
        }
    }

    println!("{}", &sv);
    println!("part one final occupied seat count {}", &sv.occupied_seat_count());
}

#[allow(dead_code)]
fn part_two() {
    let mut sv = parse_input("../input/11-input.txt");
    let mut changed = true;
    println!("{}", &sv);
    println!();

    while changed {
        changed = false;
        let mut ts = vec![Seat::Floor; sv.seats.len()];

        for idx in 0..sv.seats.len() {
            match sv.seats.get(idx) {
                Some(Seat::Empty) if sv.visible_occupied_count(idx) == 0 => {
                    ts[idx] = Seat::Occupied;
                    changed = true;
                },
                Some(Seat::Occupied) if sv.visible_occupied_count(idx) >= 5 => {
                    ts[idx] = Seat::Empty;
                    changed = true;
                },
                _ => ts[idx] = sv.seats[idx],
            }
        }
        if changed {
            sv.seats = ts;
        }
    }

    println!("{}", &sv);
    println!("part two final occupied seat count {}", &sv.occupied_seat_count());
}

fn main() {
    part_one();
    part_two()
}










#[cfg(test)]
mod tests {
    use crate::{Seat, SeatVec, parse_input};

    #[test]
    fn get_upper_left_adjacents_indices() {
        let v = vec![
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
            Seat::Floor,Seat::Floor,Seat::Floor,Seat::Floor,
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
        ];
        let sv = SeatVec::new(v, 4);
        let ul = sv.adjacent_indices(0);
        assert_eq!(ul.len(), 3);
        assert!(ul.contains(&1));
        assert!(ul.contains(&4));
        assert!(ul.contains(&5));
        assert_eq!(ul.contains(&0), false);
    }

    #[test]
    fn get_upper_right_adjacents_indices() {
        let v = vec![
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
            Seat::Floor,Seat::Floor,Seat::Floor,Seat::Floor,
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
        ];
        let sv = SeatVec::new(v, 4);
        let ul = sv.adjacent_indices(3);
        assert_eq!(ul.len(), 3);
        assert!(ul.contains(&2));
        assert!(ul.contains(&6));
        assert!(ul.contains(&7));
        assert_eq!(ul.contains(&3), false);
    }

    #[test]
    fn get_top_row_adjacent_indices() {
        let v = vec![
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
            Seat::Floor,Seat::Floor,Seat::Floor,Seat::Floor,
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
        ];
        let sv = SeatVec::new(v, 4);
        let ul = sv.adjacent_indices(1);
        assert_eq!(ul.len(), 5);
        assert!(ul.contains(&0));
        assert!(ul.contains(&2));
        assert!(ul.contains(&4));
        assert!(ul.contains(&5));
        assert!(ul.contains(&6));
        assert_eq!(ul.contains(&1), false);
    }

    #[test]
    fn get_middle_adjacent_indices() {
        let v = vec![
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
            Seat::Floor,Seat::Floor,Seat::Floor,Seat::Floor,
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
        ];
        let sv = SeatVec::new(v, 4);
        let ul = sv.adjacent_indices(5);
        assert_eq!(ul.len(), 8);
        assert!(ul.contains(&0));
        assert!(ul.contains(&1));
        assert!(ul.contains(&2));
        assert!(ul.contains(&4));
        assert!(ul.contains(&6));
        assert!(ul.contains(&8));
        assert!(ul.contains(&9));
        assert!(ul.contains(&10));
        assert_eq!(ul.contains(&5), false);
    }

    #[test]
    fn get_left_col_adjacent_indices() {
        let v = vec![
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
            Seat::Floor,Seat::Floor,Seat::Floor,Seat::Floor,
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
        ];
        let sv = SeatVec::new(v, 4);
        let ul = sv.adjacent_indices(4);
        assert_eq!(ul.len(), 5);
        assert!(ul.contains(&1));
        assert_eq!(ul.contains(&4), false);
    }

    #[test]
    fn get_right_col_adjacent_indices() {
        let v = vec![
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
            Seat::Floor,Seat::Floor,Seat::Floor,Seat::Floor,
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
        ];
        let sv = SeatVec::new(v, 4);
        let ul = sv.adjacent_indices(7);
        assert_eq!(ul.len(), 5);
        assert!(ul.contains(&6));
        assert_eq!(ul.contains(&7), false);
    }

    #[test]
    fn get_bottom_row_adjacent_indices() {
        let v = vec![
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
            Seat::Floor,Seat::Floor,Seat::Floor,Seat::Floor,
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
        ];
        let sv = SeatVec::new(v, 4);
        let ul = sv.adjacent_indices(9);
        assert_eq!(ul.len(), 5);
        assert!(ul.contains(&5));
        assert!(ul.contains(&6));
        assert!(ul.contains(&10));
        assert!(ul.contains(&4));
        assert!(ul.contains(&8));
        assert_eq!(ul.contains(&9), false);
    }

    #[test]
    fn get_bottom_left_adjacent_indices() {
        let v = vec![
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
            Seat::Floor,Seat::Floor,Seat::Floor,Seat::Floor,
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
        ];
        let sv = SeatVec::new(v, 4);
        let ul = sv.adjacent_indices(8);
        assert_eq!(ul.len(), 3);
        assert!(ul.contains(&4));
        assert!(ul.contains(&5));
        assert!(ul.contains(&9));
        assert_eq!(ul.contains(&8), false);
    }

    #[test]
    fn get_bottom_right_adjacent_indices() {
        let v = vec![
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
            Seat::Floor,Seat::Floor,Seat::Floor,Seat::Floor,
            Seat::Empty,Seat::Empty,Seat::Empty,Seat::Empty,
        ];
        let sv = SeatVec::new(v, 4);
        let ul = sv.adjacent_indices(11);
        assert_eq!(ul.len(), 3);
        assert!(ul.contains(&7));
        assert!(ul.contains(&10));
        assert!(ul.contains(&6));
        assert_eq!(ul.contains(&11), false);
    }

    // #[test]
    // fn test_empty_seat_up_right() {
    //     let mut sv = parse_input("../input/11-ex1.txt");
    //     let empty_seats = sv.empty_seat_up_right(90);
    //     dbg!(&empty_seats);
    // }

}