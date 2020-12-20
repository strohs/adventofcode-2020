// Day 7 - Handy Haversacks
// https://adventofcode.com/2020/day/7

use lazy_static::lazy_static;
use regex::Regex;
use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct Bag {
    // number of bags contained, = 0 if this bag is a container bag
    amount: i32,
    // name of the container bag, or name of the bag being contained
    name: String,
}

impl Bag {
    fn new(amount: i32, name: String) -> Self {
        Bag { amount, name }
    }
}

// Bags are equal if their names are equal
impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Bag {}

// only the Bag.name is hashed
impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}




/// returns an iterator over the lines of the file pointed to by filename
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_container_bag(line: &str) -> Bag {
    lazy_static! {
        static ref CONTAINER_RE: Regex =
            Regex::new(r"(.+?) bag[s]?").expect("valid RegEx");
    }
    // get the container bag name
    let container: String = CONTAINER_RE.captures(line).unwrap()[1].to_string();
    Bag::new(0, container)
}

fn parse_contained_bags(line: &str) -> Option<Vec<Bag>> {
    lazy_static! {
        static ref BAGS_RE: Regex = Regex::new(r"(\d+) (.+?) bag[s]?").expect("valid regex");
    }
    if line.find("no other").is_some() {
        return None
    }
    let bags = BAGS_RE.captures_iter(line)
        .map(|cap| {
            let amount: i32 = cap[1].parse().expect("valid integer for bag amount");
            let name: String = cap[2].to_string();
            Bag::new(amount, name)
        })
        .collect::<Vec<Bag>>();

    Some(bags)
}

// compute count of unique bag colors can eventually contain at least one shiny gold bag
fn part_one() {
    // a HashMap that maps a contained bag name, to a set of bags that contain them
    let mut bag_map: HashMap<String, HashSet<Bag>> = HashMap::new();

    // read the input file and parse it into the bag_map
    for res_line in read_lines("../input/07-input.txt").unwrap() {
        let line = res_line.unwrap();
        let container = parse_container_bag(&line);
        let contained = parse_contained_bags(&line);

        // build a HashMap that maps each contained bag name, to a set of bags that contain them
        if let Some(contained_bags) = contained {
            for bag in contained_bags {
                let bentry = bag_map.entry(bag.name).or_insert(HashSet::new());
                bentry.insert(container.to_owned());
            }
        }
    }

    // now determine the count of bags that can contain at least one 'shiny gold' bag
    //
    let mut bags_to_visit = bag_map.get("shiny gold").unwrap().iter().collect::<Vec<&Bag>>();
    let mut containing_bags: HashSet<&str> = bags_to_visit.iter().map(|&b| &*b.name).collect();
    while !bags_to_visit.is_empty() {
        let next = bags_to_visit.pop().unwrap();
        //println!("next is {:?} with values {:?}", &next, &bag_map.get(&*next.name));
        if let Some(next_bags) = bag_map.get(&*next.name) {
            for b in next_bags {
                containing_bags.insert(&*b.name);
                bags_to_visit.push(b);
            }
        }

    }
    //dbg!(&containing_bags);
    println!("bag colors that can eventually contain at least one shiny gold bag = {}", &containing_bags.len());
}

// parse the input file into a HashMap
fn parse_input(filename: &str) -> HashMap<Bag, Vec<Bag>> {
    let mut bag_map = HashMap::new();
    for res_line in read_lines(filename).unwrap() {
        let line = res_line.unwrap();
        let contained = parse_contained_bags(&line);
        if let Some(contained_bags) = contained {
            let contained_sum = contained_bags.iter().map(|b| b.amount).sum();
            let mut container = parse_container_bag(&line);
            container.amount = contained_sum;
            bag_map.insert(container, contained_bags);
        }
    }
    bag_map
}

// recursively count the amount of bags contained within `current` bag
fn sum_bag(current: &Bag, map: &HashMap<Bag, Vec<Bag>>) -> usize {
    let mut sum = 0;
    if let Some(vec) = map.get(current) {
        for next_bag in vec.iter() {
            sum += next_bag.amount as usize + next_bag.amount as usize * sum_bag(next_bag, map);
        }
    }
    sum
}

// how many individual bags are required inside your shiny gold bag
fn part_two() {
    // map input file into a hash map
    let bags = parse_input("../input/07-input.txt");

    // now determine the count of individual bags required inside a 'shiny gold' bag
    let (bag, _) = bags.get_key_value(&Bag::new(0, "shiny gold".to_string())).unwrap();

    //dbg!(&bags);

    let total = sum_bag(bag, &bags);
    println!("total = {}", total);
}


fn main() {
    //part_one();
    part_two();
}


#[cfg(test)]
mod tests {
    use crate::{parse_container_bag, parse_contained_bags};

    #[test]
    fn can_parse_containing_bag_name() {
        let line = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let bag = parse_container_bag(line);
        assert_eq!(bag.name, "light red");
        assert_eq!(bag.amount, 0);
    }

    #[test]
    fn can_parse_two_contained_bags() {
        let line = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let bags = parse_contained_bags(line);
        assert!(bags.is_some());
        assert_eq!(bags.as_ref().unwrap()[0].amount, 1);
        assert_eq!(bags.as_ref().unwrap()[0].name, "bright white");
        assert_eq!(bags.as_ref().unwrap()[1].amount, 2);
        assert_eq!(bags.as_ref().unwrap()[1].name, "muted yellow");
    }

    #[test]
    fn can_parse_no_contained_bags() {
        let line = "light red bags contain no other bags.";
        let bags = parse_contained_bags(line);
        assert!(bags.is_none());
    }
}