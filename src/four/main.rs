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

    let passports = PASSPORT_SEP.split(&file).collect::<Vec<_>>();

    println!(
        "{}",
        passports
            .iter()
            .map(|x| validate(*x))
            .collect::<Counter<_>>()[&true]
    );

    Ok(())
}

fn validate(passport: &str) -> bool {
    lazy_static! {
        static ref FIELDS_RE: Regex = Regex::new(r"(\w+):(\S+)\s*").unwrap();
    }

    let captures_iter = FIELDS_RE.captures_iter(passport);
    let mut fields: HashMap<String, String> = HashMap::new();

    for capture in captures_iter {
        fields.insert(
            capture.get(1).unwrap().as_str().to_owned(),
            capture.get(2).unwrap().as_str().to_owned(),
        );
    }

    validate_fields(fields)
}

fn validate_fields(fields: HashMap<String, String>) -> bool {
    let keys = fields
        .iter()
        .filter(|(k, v)| validate_field(k, v))
        .map(|(k, _)| k)
        .map(|x| x.into())
        .collect::<HashSet<String>>();

    let required_keys: HashSet<String> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|x| x.to_string())
        .collect::<HashSet<_>>();

    required_keys.intersection(&keys).collect::<Vec<_>>().len() == required_keys.len()
}

fn validate_field(_: &str, _: &str) -> bool {
    true
}
