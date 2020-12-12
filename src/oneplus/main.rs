use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/oneplus/input")?;

    let items: HashSet<i32> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect::<HashSet<i32>>();

    for item in items.clone() {
        let tt_remainder = 2020 - item;

        match sums_to_x(items.clone(), tt_remainder) {
            None => (),
            Some((fst, snd)) => {
                println!("{} x {} x {} = {}", item, fst, snd, item * fst * snd);
                break;
            }
        }
    }

    Ok(())
}

fn sums_to_x(items: HashSet<i32>, x: i32) -> Option<(i32, i32)> {
    for item in items.clone() {
        let remainder = x - item;

        if items.contains(&remainder) {
            return Some((item, remainder));
        }
    }

    None
}
