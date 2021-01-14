// Advent of Code - Day 12 - Rain Risk part 1
// https://adventofcode.com/2020/day/12

use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::io::BufRead;
use crate::NavInstr::{Left, Right, North, South, East, West, Forward};

#[derive(Debug, PartialEq)]
/// Navigation Instruction
enum NavInstr {
    North(u32),
    South(u32),
    East(u32),
    West(u32),
    Left(u32),
    Right(u32),
    Forward(u32),
}

impl TryFrom<&String> for NavInstr {
    type Error = &'static str;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let nidx = s.find(char::is_numeric)
            .ok_or("nav instruction must have an integer amount")?;
        let (command, amount) = s.split_at(nidx);
        let amount = amount
            .parse::<u32>()
            .map_err(|_pe| "could not parse nav instr into a valid u32")?;

        match command {
            "N" => Ok(NavInstr::North(amount)),
            "S" => Ok(NavInstr::South(amount)),
            "E" => Ok(NavInstr::East(amount)),
            "W" => Ok(NavInstr::West(amount)),
            "L" => Ok(NavInstr::Left(amount)),
            "R" => Ok(NavInstr::Right(amount)),
            "F" => Ok(NavInstr::Forward(amount)),
            _ => Err("Navigation instruction must be a valid letter")
        }
    }
}

#[derive(Debug)]
struct Ship {
    // ships current heading, 0 = North, E = 90, S = 180, W = 270
    heading: u32,
    // ships current East/West Position, positive values are East, negative values are West
    e_w_pos: i32,
    // ships current North/South position. Positive values are North, negative values are south
    n_s_pos: i32,
}

impl Ship {
    fn new() -> Self {
        Self {
            heading: 90,
            e_w_pos: 0,
            n_s_pos: 0,
        }
    }

    /// turn ship left or right by some amount, onto a new heading
    fn turn_ship(&mut self, ni: &NavInstr) {
        let new_heading = match ni {
            Left(amount) => {
                let mut nh = -(*amount as i32) + self.heading as i32;
                if nh < 0 {
                    nh = 360 + nh;
                }
                nh as u32
            },
            Right(amount) => {
                let mut nh = self.heading + *amount;
                if nh >= 360 {
                    nh = nh - 360;
                }
                nh as u32
            },
            instr => panic!("unknown turn instruction {:?}", instr)
        };
        self.heading = new_heading;
    }

    /// move ship in the specified direction
    fn move_ship(&mut self, ni: &NavInstr) {
        match ni {
            North(amt) => self.n_s_pos += *amt as i32,
            South(amt) => self.n_s_pos -= *amt as i32,
            East(amt) => self.e_w_pos += *amt as i32,
            West(amt) => self.e_w_pos -= *amt as i32,
            Forward(amt) => {
                match self.heading {
                    0 => self.n_s_pos += *amt as i32,
                    180 => self.n_s_pos -= *amt as i32,
                    90 => self.e_w_pos += *amt as i32,
                    270 => self.e_w_pos -= *amt as i32,
                    other => panic!("forward movement was not in a N,S,E,W direction {}", other),
                }
            },
            turn => panic!("invalid move instruction {:?}", turn),
        }
    }

    /// returns the manhattan distance between the ships current position and its origin
    fn manhattan_distance(&self) -> u32 {
        (self.e_w_pos.abs() + self.n_s_pos.abs()) as u32
    }
}

/// parse input into a Vector of nav instructions
fn parse_input(filename: &str) -> Vec<NavInstr> {
    let file = File::open(filename).unwrap();

    io::BufReader::new(file)
        .lines()
        .map(|line| {
            let nav_str = line.unwrap();
            let ni = NavInstr::try_from(&nav_str).expect("input is well-formed");
            ni
        })
        .collect::<Vec<NavInstr>>()
}

fn main() {
    let nis = parse_input("../input/12-input.txt");
    let mut ship = Ship::new();
    dbg!(&ship);

    for ni in &nis {
        match ni {
            Right(_) | Left(_) => ship.turn_ship(ni),
            _ => ship.move_ship(ni),
        }
    }
    println!("ship is at {:?} with a manhattan distance of {}",&ship, &ship.manhattan_distance());
}

#[cfg(test)]
mod tests {
    use crate::{NavInstr, Ship};
    use std::convert::TryFrom;

    #[test]
    fn parse_nav_instruction() {
        let nis = String::from("F145");
        let ni = NavInstr::try_from(&nis);
        assert_eq!(ni.unwrap(), NavInstr::Forward(145));
    }

    #[test]
    fn turn_ship_right() {
        let mut ship = Ship::new();
        ship.turn_ship(&NavInstr::Right(180));
        assert_eq!(ship.heading, 270);
    }

    #[test]
    fn turn_ship_right_270() {
        let mut ship = Ship::new();
        ship.turn_ship(&NavInstr::Right(270));
        assert_eq!(ship.heading, 0);
    }

    #[test]
    fn turn_ship_left_180() {
        let mut ship = Ship::new();
        ship.turn_ship(&NavInstr::Left(180));
        assert_eq!(ship.heading, 270);
    }

    #[test]
    fn turn_ship_left_90() {
        let mut ship = Ship::new();
        ship.turn_ship(&NavInstr::Left(90));
        assert_eq!(ship.heading, 0);
    }

    #[test]
    fn turn_ship_left_270() {
        let mut ship = Ship::new();
        ship.turn_ship(&NavInstr::Left(270));
        assert_eq!(ship.heading, 180);
    }
}