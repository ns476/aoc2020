use bounded_vec_deque::BoundedVecDeque;
use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let elems: Vec<u64> = std::fs::read_to_string("src/fifteen/input")
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    dbg!(NumberGame::from_starting_numbers(elems).nth(30_000_000 - 1));

    Ok(())
}

#[derive(Debug)]
struct NumberGame {
    starting_numbers: Vec<u64>,
    turn: u64,
    maybe_last_number_spoken: Option<u64>,
    indices_by_number: HashMap<u64, BoundedVecDeque<u64>>,
}

impl NumberGame {
    fn from_starting_numbers(starting_numbers: Vec<u64>) -> NumberGame {
        NumberGame {
            starting_numbers,
            turn: 1,
            maybe_last_number_spoken: None,
            indices_by_number: HashMap::new(),
        }
    }
}

impl Iterator for NumberGame {
    type Item = u64;

    fn next(&mut self) -> std::option::Option<Self::Item> {
        let initial_item = self.starting_numbers.get(self.turn as usize - 1).copied();

        let items_after = self.maybe_last_number_spoken.map(|last_spoken| {
            let indices_spoken = self.indices_by_number.get(&last_spoken).unwrap();

            match indices_spoken.len() {
                1 => 0,
                2 => indices_spoken[1] - indices_spoken[0],
                _ => panic!("Zero-length deque encountered!"),
            }
        });

        let item = initial_item.or(items_after).unwrap();

        let entry = self
            .indices_by_number
            .entry(item)
            .or_insert_with(|| BoundedVecDeque::new(2));
        entry.push_back(self.turn);
        self.turn += 1;
        self.maybe_last_number_spoken = Some(item);

        Some(item)
    }
}
