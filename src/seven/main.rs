#[macro_use]
extern crate lazy_static;
extern crate regex;

use counter::Counter;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/four/input")?;

    lazy_static! {
        static ref PASSPORT_SEP: Regex = Regex::new("\n\n").unwrap();
    }

    let file: String = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    println!(
        "{}",
        passports
            .iter()
            .map(|x| validate(*x))
            .collect::<Counter<_>>()[&true]
    );

    Ok(())
}



#[derive(Debug)]
struct BagSpec {
    name: String,
    bags_inside: HashMap<String, i32>
}

fn parse_bag(bag_description: &str) -> BagSpec {
    let top_split = bag_description.split(" bags contain ").collect::<Vec<_>>();

    let bag_name = top_split[0];

    // 2 shiny gold bags, 9 faded blue bags.
    let bags = top_split[1];

    lazy_static!{
        static ref BAG_RE: Regex = Regex::new("(\d+) (.+) bag(s)?").unwrap();
    }
}