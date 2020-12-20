use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

/// return an iterator over the lines of the file pointed to by filename
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// wrapping get. If the column index `c` is >= the length of a the current row, then this function
/// will wrap to the beginning of the row
fn wget(sl: &Vec<Vec<char>>, r: usize, c: usize) -> Option<&char> {
    match sl.get(r).unwrap().len() {
        0 => None,
        n if c >= n => sl.get(r).unwrap().get(c % n),
        _ => sl.get(r).unwrap().get(c),
    }
}

fn tree_count(slope: &Vec<Vec<char>>, dr: usize, dc: usize) -> usize {
    let tree = '#';

    let mut tree_count = 0;
    let mut r = 0;
    let mut c = 0;
    while r < slope.len() {
        if *wget(&slope, r, c).unwrap() == tree {
            tree_count += 1;
        }
        r += dr;
        c += dc;
    }
    tree_count
}

fn main() {
    // read all lines of input into a Vec<Vec<char>>
    let slope: Vec<Vec<char>> = read_lines("../input/03-input.txt")
        .unwrap()
        .into_iter()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect();

    let tests = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let results: Vec<usize> = tests
        .iter()
        .map(|&(dr, dc)| tree_count(&slope, dr, dc))
        .collect();

    dbg!(&results);
    let prod = results.iter().fold(1, |acc, n| acc * *n);
    println!("product = {}", prod);
}
