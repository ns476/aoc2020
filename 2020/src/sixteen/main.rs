#[macro_use]
extern crate lazy_static;
extern crate counter;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use counter::Counter;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/sixteen/input")?;
    let lines = BufReader::new(file).lines().map(|x| x.unwrap());

    lazy_static! {
        static ref BOUND_RE: Regex = Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        static ref TICKET_RE: Regex = Regex::new(r"^([0-9,]+)+$").unwrap();
    }

    let mut bounds: HashMap<String, ValidRanges> = HashMap::new();
    let mut tickets: Vec<Vec<u64>> = Vec::new();

    for line in lines {
        if let Some(capt) = BOUND_RE.captures(&line) {
            bounds.insert(
                capt.get(1).unwrap().as_str().to_string(),
                ValidRanges {
                    ranges: vec![
                        capt.get(2).unwrap().as_str().parse::<u64>().unwrap()
                            ..=capt.get(3).unwrap().as_str().parse::<u64>().unwrap(),
                        capt.get(4).unwrap().as_str().parse::<u64>().unwrap()
                            ..=capt.get(5).unwrap().as_str().parse::<u64>().unwrap(),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                },
            );
        }

        if TICKET_RE.find(&line).is_some() {
            tickets.push(line.split(',').map(|x| x.parse::<u64>().unwrap()).collect());
        }
    }

    let mut tickets_iter = tickets.iter();

    // Skip our ticket
    tickets_iter.next();

    dbg!(&bounds);

    let invalid_tickets = tickets_iter
        .map(|fields| {
            return fields.iter().find(|&x| {
                return !bounds
                    .values()
                    .any(|valid_range| valid_range.any_is_valid(*x));
            });
        })
        .flatten();

    dbg!(invalid_tickets.sum::<u64>());

    Ok(())
}

#[derive(Debug)]
struct ValidRanges {
    ranges: HashSet<std::ops::RangeInclusive<u64>>,
}

impl ValidRanges {
    fn any_is_valid(&self, x: u64) -> bool {
        self.ranges.iter().any(|range| range.contains(&x))
    }
}
