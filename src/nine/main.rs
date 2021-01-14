extern crate bounded_vec_deque;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use bounded_vec_deque::BoundedVecDeque;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/nine/input")?;

    let stream = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse::<i32>().unwrap());

    let preamble_size = 25;

    let mut deque: BoundedVecDeque<i32> = BoundedVecDeque::new(preamble_size);

    for elem in stream {
        if deque.is_full() {
            let elems_in_window: HashSet<i32> = deque.clone().into_iter().collect();

            if !is_sum(&elems_in_window, elem) {
                dbg!(elem);
                break;
            }
        }

        deque.push_back(elem);
    }

    Ok(())
}

fn is_sum(elems_in_window: &HashSet<i32>, elem: i32) -> bool {
    for x in elems_in_window {
        if elems_in_window.contains(&(elem - x)) {
            return true;
        }
    }

    false
}
