use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("../input/06-input.txt")?;

    let groups: Vec<Vec<String>> =
        io::BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .fold(vec![vec![]], |mut grp, line| {
                match line.len() {
                    0 => grp.push(vec![]),
                    _ => grp.last_mut().unwrap().push(line),
                }
                grp
            });

    dbg!(&groups);

    Ok(())
}
