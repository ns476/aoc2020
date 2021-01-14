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

    let file: String = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    let passports = file.split("\n\n").collect::<Vec<_>>();

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
    println!("fields {:?}", fields);

    let valid_keys = fields
        .iter()
        .filter(|(k, v)| validate_field(k, v))
        .map(|(k, _)| k)
        .map(|x| x.into())
        .collect::<HashSet<String>>();

    let required_keys: HashSet<String> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|x| x.to_string())
        .collect::<HashSet<_>>();

    let valid = required_keys
        .intersection(&valid_keys)
        .count()
        == required_keys.len();

    println!("valid keys {:?} valid {:?}", valid_keys, valid);

    valid
}

fn validate_field(k: &str, v: &str) -> bool {
    match k {
        "byr" => v
            .parse::<i32>()
            .ok()
            .filter(|&x| x >= 1920 && x <= 2002)
            .is_some(),
        "iyr" => v
            .parse::<i32>()
            .ok()
            .filter(|&x| x >= 2010 && x <= 2020)
            .is_some(),
        "eyr" => v
            .parse::<i32>()
            .ok()
            .filter(|&x| x >= 2020 && x <= 2030)
            .is_some(),
        "hgt" => {
            lazy_static! {
                static ref HEIGHT_RE: Regex = Regex::new(r"^(\d+)((in)|(cm))$").unwrap();
            }

            let height = HEIGHT_RE
                .captures(v)
                .and_then(|x| x.get(1))
                .map(|x| x.as_str())
                .and_then(|x| x.parse::<u32>().ok());
            let cm_or_in = HEIGHT_RE
                .captures(v)
                .and_then(|x| x.get(2))
                .map(|x| x.as_str());

            match cm_or_in {
                Some("cm") => height.filter(|&x| x >= 150 && x <= 193).is_some(),
                Some("in") => height.filter(|&x| x >= 59 && x <= 76).is_some(),
                _ => false,
            }
        }
        "hcl" => {
            lazy_static! {
                static ref COLOR_RE: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
            }

            COLOR_RE.is_match(v)
        }
        "ecl" => {
            lazy_static! {
                static ref ALLOWED_COLORS: HashSet<String> =
                    vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                        .iter()
                        .map(|&x| String::from(x))
                        .collect();
            }

            ALLOWED_COLORS.contains(v)
        }
        "pid" => {
            lazy_static! {
                static ref PID_RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
            }

            PID_RE.is_match(v)
        }
        &_ => false,
    }
}
