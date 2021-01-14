use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/one/input")?;

    let items: HashSet<i32> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect::<HashSet<i32>>();

    match sums_to_x(items, 2020) {
        None => println!("Nothing sums to 2020"),
        Some((fst, snd)) => println!("{} x {} = {}", fst, snd, fst * snd),
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
