extern crate bounded_vec_deque;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use bounded_vec_deque::BoundedVecDeque;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/nine/input")?;

    let elems = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    let weak_number = find_weak_number(&elems);
    let sequence = find_sequence(&elems, weak_number.unwrap()).unwrap();

    dbg!([sequence.iter().min().unwrap() + sequence.iter().max().unwrap()]);

    Ok(())
}

fn is_sum(elems_in_window: &HashSet<i64>, elem: i64) -> bool {
    for x in elems_in_window {
        if elems_in_window.contains(&(elem - x)) {
            return true;
        }
    }

    false
}

fn find_weak_number(elems: &[i64]) -> Option<i64> {
    let preamble_size = 25;
    let mut deque: BoundedVecDeque<i64> = BoundedVecDeque::new(preamble_size);

    for elem in elems {
        if deque.is_full() {
            let elems_in_window: HashSet<i64> = deque.clone().into_iter().collect();

            if !is_sum(&elems_in_window, *elem) {
                return Some(*elem);
            }
        }

        deque.push_back(*elem);
    }

    None
}

fn find_sequence(elems: &[i64], weak_number: i64) -> Option<Vec<i64>> {
    let mut slice: &[i64] = &[];

    for i in 0..elems.len() {
        for j in (i + 1)..elems.len() {
            slice = &elems[i..j];

            if slice.iter().sum::<i64>() >= weak_number {
                break;
            }
        }

        if slice.iter().sum::<i64>() == weak_number {
            return Some(slice.to_vec());
        }
    }

    None
}
