extern crate counter;

use counter::Counter;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/eleven/input")?;

    let elems: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let mut cur = elems;

    loop {
        let next = iterate(&cur);

        if next == cur {
            break;
        }

        cur = next;
    }

    let occupied_count = cur.iter().flatten().cloned().collect::<Counter<char>>()[&'#'];
    dbg!(occupied_count);

    Ok(())
}

fn iterate(elems: &[Vec<char>]) -> Vec<Vec<char>> {
    let h = elems.len();
    let w = elems[0].len();

    let mut next = elems.to_vec();

    for i in 0..h {
        for j in 0..w {
            let occupied_count = neighbours(elems, i as i64, j as i64)
                .iter()
                .cloned()
                .collect::<Counter<char>>()[&'#'];

            let cell_next = match elems[i][j] {
                'L' if occupied_count == 0 => '#',
                '#' if occupied_count >= 5 => 'L',
                x => x,
            };

            next[i][j] = cell_next;
        }
    }

    next
}

fn get<T: Clone>(vec: &[T], i: i64) -> Option<&T> {
    if i < 0 {
        return None;
    }

    vec.get(i as usize)
}

fn get_in_dir<F>(elems: &[Vec<char>], i: i64, j: i64, next: F) -> Option<&char>
where
    F: Fn(i64, i64) -> (i64, i64),
{
    fn inner<F>(elems: &[Vec<char>], i: i64, j: i64, next: F) -> Option<&char>
    where
        F: Fn(i64, i64) -> (i64, i64),
    {
        match get(elems, i).and_then(|row| get(row, j)) {
            Some('.') => {
                let (next_i, next_j) = next(i, j);
                inner(elems, next_i, next_j, next)
            }
            x => x,
        }
    }

    let (next_i, next_j) = next(i, j);

    inner(elems, next_i, next_j, next)
}

fn neighbours(elems: &[Vec<char>], i: i64, j: i64) -> Vec<char> {
    std::iter::empty()
        .chain(get_in_dir(elems, i, j, |x, y| (x - 1, y - 1)).iter())
        .chain(get_in_dir(elems, i, j, |x, y| (x - 1, y)).iter())
        .chain(get_in_dir(elems, i, j, |x, y| (x - 1, y + 1)).iter())
        .chain(get_in_dir(elems, i, j, |x, y| (x, y - 1)).iter())
        .chain(get_in_dir(elems, i, j, |x, y| (x, y + 1)).iter())
        .chain(get_in_dir(elems, i, j, |x, y| (x + 1, y - 1)).iter())
        .chain(get_in_dir(elems, i, j, |x, y| (x + 1, y)).iter())
        .chain(get_in_dir(elems, i, j, |x, y| (x + 1, y + 1)).iter())
        .cloned()
        .cloned()
        .collect()
}
