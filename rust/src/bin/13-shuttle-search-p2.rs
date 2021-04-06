use std::fs::File;
use std::io;
use std::io::BufRead;
use num_bigint::BigInt;
use num_traits::{Zero};

type BusIds = Vec<Option<u32>>;

/// parse input into a Vector of Option<i32>
fn parse_input(filename: &str) -> BusIds {
    let file = File::open(filename).unwrap();
    let mut bus_ids: BusIds = vec![];
    let buf_reader = io::BufReader::new(file);

    for (idx, line) in buf_reader.lines().enumerate() {
        let line = line.unwrap();
        // we only want the second line in the file
        if idx == 1 {
            line.split(",")
                .for_each(|s| {
                    if let Ok(i) = s.parse::<u32>() {
                        bus_ids.push(Some(i));
                    } else {
                        bus_ids.push(None);
                    }
                });
        }
    }
    bus_ids
}

fn absolute_modulo(a: isize, b: isize) -> isize {
    ((a % b) + b) % b
}


fn get_inverse(a: usize, modulo: usize) -> usize {
    let b = a % modulo;
    for i in 1..modulo {
        if (b * i) % modulo == 1 {
            return i;
        }
    }
    1
}


fn chinese_remainder(bids: BusIds) -> BigInt {
    // x =- a (mod n)
    // x - some unknown, constant value of t
    // a - bus number MINUS offset % bus number
    // n - cycle length (= bus number)

    // to solve each row, we also need
    // N - all n's added up
    // nU = N / n
    // i - inverse modulo

    // multiply all busIDs together and store them in N
    let N: usize = bids
        .iter()
        .filter(|&&b| b.is_some())
        .map(|b| b.unwrap() as usize)
        .product();

    let sum: BigInt = bids
        .iter()
        .enumerate()
        .fold(Zero::zero(), |acc, (idx, id)|{
            if let Some(cur) = *id {
                let a = absolute_modulo( cur as isize - idx as isize, cur as isize);
                let nU = N / cur as usize;
                let inverse = get_inverse(nU, cur as usize);
                println!("x = {} (mod {})", &a, &cur);
                acc + (a * nU as isize * inverse as isize)
            } else {
                acc
            }
    });
    sum % N
}


fn main() {
    let bus_ids = parse_input("../input/13-input.txt");
    let answer = chinese_remainder(bus_ids);
    dbg!(answer);
}