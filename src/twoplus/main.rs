#[macro_use]
extern crate lazy_static;
extern crate regex;

use counter::Counter;
use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/two/input")?;

    let valid_count: usize = BufReader::new(file)
        .lines()
        .map(|x| parse_and_check(x.unwrap()))
        .collect::<Counter<bool>>()[&true];

    println!("{}", valid_count);

    Ok(())
}

fn parse_and_check(str: String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([0-9]+)\-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
    }
    let captures = RE.captures(&str).unwrap();

    let min: usize = captures.get(1).unwrap().as_str().parse().unwrap();
    let max: usize = captures.get(2).unwrap().as_str().parse().unwrap();
    let chr = captures.get(3).unwrap().as_str().chars().nth(0).unwrap();

    let pwd = captures.get(4).unwrap().as_str();

    check(min, max, chr, pwd)
}

fn check(min: usize, max: usize, chr: char, pwd: &str) -> bool {
    let first_present = pwd.chars().nth(min - 1).unwrap() == chr;
    let second_present = pwd.chars().nth(max - 1).unwrap() == chr;

    first_present ^ second_present
}
