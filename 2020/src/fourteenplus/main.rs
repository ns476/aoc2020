#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/fourteen/input")?;

    let elems: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();

    let mut mask = MemMask::no_effect();

    let mut mem: HashMap<u64, u64> = HashMap::new();

    lazy_static! {
        static ref MASK_RE: Regex = Regex::new(r"mask = ([X10]+)").unwrap();
        static ref MEM_RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    }

    for line in elems {
        dbg!(&line);
        if let Some(mask_groups) = MASK_RE.captures_iter(&line).next() {
            mask = MemMask::from_string(mask_groups.get(1).unwrap().as_str())
        }

        if let Some(mem_groups) = MEM_RE.captures_iter(&line).next() {
            let addr = mem_groups.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let val = mem_groups.get(2).unwrap().as_str().parse::<u64>().unwrap();

            mask.apply(addr).iter().for_each(|addr| {
                mem.insert(*addr, val);
            })
        }
    }

    dbg!(mem.values().sum::<u64>());

    Ok(())
}

#[derive(Debug)]
struct MemMask {
    floating_masks: HashSet<Mask>,
    or_mask: u64,
}

impl MemMask {
    fn no_effect() -> MemMask {
        MemMask {
            floating_masks: std::iter::once(Mask::no_effect()).collect(),
            or_mask: 0,
        }
    }

    fn from_string(mask: &str) -> MemMask {
        let mut or_mask: u64 = 0;
        let mut floating_bits: HashSet<u64> = HashSet::new();

        for (i, bit) in mask.chars().rev().enumerate() {
            match bit {
                '1' => or_mask |= 1 << i,
                'X' => {
                    floating_bits.insert(i as u64);
                }
                _ => (),
            }
        }

        MemMask {
            floating_masks: MemMask::float_masks(&floating_bits),
            or_mask,
        }
    }

    fn float_masks(bits: &HashSet<u64>) -> HashSet<Mask> {
        if bits.is_empty() {
            return std::iter::once(Mask::no_effect()).collect();
        }

        bits.iter()
            .map(|&bit| {
                let mut other_bits = bits.clone();
                other_bits.remove(&bit);

                let masks_without_bit = MemMask::float_masks(&other_bits);

                masks_without_bit
                    .iter()
                    .map(|mask| mask.with_bit_set_to_1(bit))
                    .chain(
                        masks_without_bit
                            .iter()
                            .map(|mask| mask.with_bit_set_to_0(bit)),
                    )
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }

    fn float_bits(&self, addr: u64) -> HashSet<u64> {
        self.floating_masks
            .iter()
            .map(|mask| mask.apply(addr))
            .collect()
    }

    fn apply(&self, addr: u64) -> HashSet<u64> {
        self.float_bits(addr | self.or_mask)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
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

    fn apply(&self, num: u64) -> u64 {
        (num & self.and_mask) | self.or_mask
    }

    fn with_bit_set_to_1(&self, index: u64) -> Mask {
        let mut copy = *self;
        copy.or_mask |= 1 << index;
        copy
    }

    fn with_bit_set_to_0(&self, index: u64) -> Mask {
        let mut copy = *self;
        copy.and_mask &= !(1 << index);
        copy
    }
}
