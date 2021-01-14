use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/ten/input")?;

    let elems = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    let jolts = build_jolts(&elems);

    let mut ones = 0;
    let mut threes = 0;

    for window in jolts.windows(2) {
        if let [l, h] = window {
            dbg!(l, h);
            if h - l == 1 {
                ones += 1;
            }

            if h - l == 3 {
                threes += 1;
            }
        }
    }

    dbg!(ones);
    dbg!(threes);
    dbg!(ones * threes);

    Ok(())
}

fn build_jolts(elems: &[i64]) -> Vec<i64> {
    let max: i64 = *(elems.iter().max().unwrap());

    let mut jolts: Vec<i64> = std::iter::once(&0)
        .chain(elems.iter())
        .chain(std::iter::once(&(max + 3)))
        .cloned()
        .collect();

    jolts.sort_unstable();

    jolts
}
