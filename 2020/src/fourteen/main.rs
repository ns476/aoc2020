#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/fourteen/input")?;

    let elems: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();

    let mut mask = Mask::no_effect();

    let mut mem: HashMap<u64, u64> = HashMap::new();

    lazy_static! {
        static ref MASK_RE: Regex = Regex::new(r"mask = ([X10]+)").unwrap();
        static ref MEM_RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    }

    for line in elems {
        if let Some(mask_groups) = MASK_RE.captures_iter(&line).next() {
            mask = Mask::from_string(mask_groups.get(1).unwrap().as_str())
        }

        if let Some(mem_groups) = MEM_RE.captures_iter(&line).next() {
            let addr = mem_groups.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let val = mem_groups.get(2).unwrap().as_str().parse::<u64>().unwrap();

            mem.insert(addr, mask.apply(val));
        }
    }

    dbg!(mem.values().sum::<u64>());

    Ok(())
}

#[derive(Debug)]
struct Mask {
    and_mask: u64,
    or_mask: u64,
}

impl Mask {
    fn no_effect() -> Mask {
        Mask {
            and_mask: !0,
            or_mask: 0,
        }
    }

    fn from_string(mask: &str) -> Mask {
        let mut and_mask: u64 = !0;
        let mut or_mask: u64 = 0;

        for (i, bit) in mask.chars().rev().enumerate() {
            match bit {
                '1' => or_mask |= 1 << i,
                '0' => and_mask &= !(1 << i),
                _ => (),
            }
        }

        Mask { and_mask, or_mask }
    }

    fn apply(&self, num: u64) -> u64 {
        (num & self.and_mask) | self.or_mask
    }
}
