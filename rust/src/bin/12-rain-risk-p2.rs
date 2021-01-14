// Advent of Code - Day 12 - Rain Risk part 2
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
struct Entity {
    // entitys current heading, 0 = North, E = 90, S = 180, W = 270
    heading: u32,
    // entitys current East/West Position, positive values are East, negative values are West
    e_w_pos: i32,
    // entitys current North/South position. Positive values are North, negative values are south
    n_s_pos: i32,
}

impl Entity {
    fn new(heading: u32, ew: i32, ns: i32) -> Self {
        Self {
            heading: heading,
            e_w_pos: ew,
            n_s_pos: ns,
        }
    }

    /// turn (rotate) this entity according to the given Navigation Instruction,
    /// around the `other` entities current position
    fn nav_turn(&mut self, ni: &NavInstr, other: &Entity) {
        let degrees = match ni {
            Left(amount) => *amount as i32,
            Right(amount) => -1 * (*amount as i32),
            instr => panic!("unknown turn instruction {:?}", instr)
        };
        let origin = (other.e_w_pos, other.n_s_pos);
        let point = (self.e_w_pos, self.n_s_pos);
        let (px, py) = Entity::rotate_point(origin, point, degrees);

        self.e_w_pos = px;
        self.n_s_pos = py;
    }

    /// move entity in the specified direction
    fn nav_move(&mut self, ni: &NavInstr) {
        match ni {
            North(amt) => self.n_s_pos += *amt as i32,
            South(amt) => self.n_s_pos -= *amt as i32,
            East(amt) => self.e_w_pos += *amt as i32,
            West(amt) => self.e_w_pos -= *amt as i32,
            other => panic!("invalid move instruction {:?}", other),
        }
    }

    /// move the entity forward by some amt, in the direction of the given entity
    fn forward(&mut self, amt: u32, other: &Entity) {

        let dx = other.e_w_pos - self.e_w_pos;
        let dy = other.n_s_pos - self.n_s_pos;
        let amt = amt as i32;

        self.e_w_pos += amt * dx;
        self.n_s_pos += amt * dy;
    }

    /// returns the manhattan distance between the entity's current position and the origin
    fn manhattan_distance(&self) -> u32 {
        (self.e_w_pos.abs() + self.n_s_pos.abs()) as u32
    }


    /// rotate `point` about `origin` point (pivot)
    /// +degrees is counter-clockwise rotation, -degrees is clockwise
    fn rotate_point(origin: (i32, i32), point: (i32, i32), degrees: i32) -> (i32, i32) {
        let angle = (degrees as f32).to_radians();

        let (px, py) = (point.0 as f32, point.1 as f32);
        let (ox, oy) = (origin.0 as f32, origin.1 as f32);

        let qx = angle.cos() * (px - ox) - angle.sin() * (py - oy) + ox;
        let qy = angle.sin() * (px - ox) + angle.cos() * (py - oy) + oy;

        (qx as i32, qy as i32)
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
    let mut ship = Entity::new(0, 0, 0);
    let mut wp = Entity::new(0, 10, 1);

    for ni in &nis {
        match ni {
            Right(_) | Left(_) => {
                wp.nav_turn(ni, &ship);
            },
            North(_) | South(_) | East(_) | West(_) => {
                wp.nav_move(ni);
            },
            Forward(amt) => {
                let wp_dx = wp.e_w_pos - ship.e_w_pos;
                let wp_dy = wp.n_s_pos - ship.n_s_pos;
                ship.forward(*amt, &wp);
                wp.e_w_pos = ship.e_w_pos + wp_dx;
                wp.n_s_pos = ship.n_s_pos + wp_dy;
            },
        }
        // println!("{:?} ship_x:{} ship_y:{}     wp_x:{} wp_y:{}", &ni, &ship.e_w_pos, &ship.n_s_pos, &wp.e_w_pos, &wp.n_s_pos);
    }
    println!("ship is at {:?} with a manhattan distance of {}",&ship, &ship.manhattan_distance());
}

#[cfg(test)]
mod tests {
    use crate::{NavInstr, Entity};
    use std::convert::TryFrom;

    #[test]
    fn parse_nav_instruction() {
        let nis = String::from("F145");
        let ni = NavInstr::try_from(&nis);
        assert_eq!(ni.unwrap(), NavInstr::Forward(145));
    }
    
    #[test]
    fn rotate_entity_right_90() {
        let (rx, ry) = Entity::rotate_point((170, 38), (180, 42), -90);
        assert_eq!(rx, 174);
        assert_eq!(ry, 28);
    }

    #[test]
    fn rotate_entity_left_90() {
        let (rx, ry) = Entity::rotate_point((0, 0), (10, 0), 90);
        assert_eq!(rx, 0);
        assert_eq!(ry, 10);
    }

    #[test]
    fn forward_4() {
        let wp = Entity::new(0, 4, 4);
        let mut ship = Entity::new(0, 2, 2);
        ship.forward(4, &wp);
        assert_eq!(ship.e_w_pos, 10);
        assert_eq!(ship.n_s_pos, 10);
    }

    #[test]
    fn forward_1_west_north() {
        let wp = Entity::new(0, -2, 4);
        let mut ship = Entity::new(0, 2, 2);
        ship.forward(1, &wp);
        assert_eq!(ship.e_w_pos, -2);
        assert_eq!(ship.n_s_pos, 4);
    }

    #[test]
    fn forward_1_west_south() {
        let wp = Entity::new(0, -4, -4);
        let mut ship = Entity::new(0, 2, 2);
        ship.forward(3, &wp);
        assert_eq!(ship.e_w_pos, -16);
        assert_eq!(ship.n_s_pos, -16);
    }

    #[test]
    fn radian_test() {
        let pos90 = 90.0_f64.to_radians();
        let neg90 = -90.0_f64.to_radians();
        println!("pos90 {}  neg90 {}", pos90, neg90);
        println!("0deg {}", 0.0_f64.to_radians());
        println!("pos180 {}  neg180 {}", 180.0_f64.to_radians(), -180.0_f64.to_radians());
    }

}