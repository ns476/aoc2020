use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/thirteen/input")?;

    let elems: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();

    let arrival_time = elems[0].parse::<usize>().unwrap();

    let mut leaving: Vec<usize> = elems[1]
        .split(',')
        .map(|x| x.to_string().parse::<usize>().ok())
        .flatten()
        .collect();

    leaving.sort_unstable();

    let mut next_departures: BTreeMap<usize, usize> = BTreeMap::new();

    for x in leaving {
        next_departures.insert(((arrival_time / x) * x) + x, x);
    }

    let (next_departure_time, next_departure_bus) = next_departures.iter().next().unwrap();

    let answer = (next_departure_time - arrival_time) * next_departure_bus;

    dbg!(answer);

    Ok(())
}
