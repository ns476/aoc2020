#[macro_use]
extern crate lazy_static;
extern crate counter;
extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::iter;

use counter::Counter;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/seventeen/input")?;
    let lines = BufReader::new(file).lines().map(|x| x.unwrap());
    let states: Vec<Vec<_>> = lines
        .map(|line| line.chars().map(CubeState::from_char).collect())
        .collect();

    let mut cubes = ConwayCubes::from_states(states);

    println!("active before starting: {}", cubes.active_count());
    for i in 1..=6 {
        cubes = cubes.next();
        println!("active on iter {}: {}", i, cubes.active_count());
    }

    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum CubeState {
    Inactive,
    Active,
}

impl CubeState {
    fn from_char(c: char) -> CubeState {
        match c {
            '.' => CubeState::Inactive,
            '#' => CubeState::Active,
            _ => panic!("Invalid cube state {}", c),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct ConwayCubes {
    data: HashMap<Coord, CubeState>,
    min_bound: Coord,
    max_bound: Coord,
}

impl ConwayCubes {
    fn from_states(states: Vec<Vec<CubeState>>) -> ConwayCubes {
        let xlen = states.get(0).map(|x| x.len()).unwrap();
        let ylen = states.len();
        let mut data: HashMap<Coord, CubeState> = HashMap::new();

        for x in 0..=(xlen - 1) {
            for y in 0..(ylen) {
                let state: CubeState = *states.get(y).unwrap().get(x).unwrap();
                data.insert(
                    Coord {
                        x: x as i64,
                        y: y as i64,
                        z: 0,
                    },
                    state,
                );
            }
        }

        ConwayCubes {
            data: data,
            min_bound: Coord { x: 0, y: 0, z: 0 },
            max_bound: Coord {
                x: (xlen as i64) - 1,
                y: (ylen as i64) - 1,
                z: 0,
            },
        }
    }

    fn current_state_for_coord(&self, coord: Coord) -> CubeState {
        *self.data.get(&coord).unwrap_or(&CubeState::Inactive)
    }

    fn neighbouring_states<'a>(&'a self, coord: Coord) -> impl Iterator<Item = CubeState> + 'a {
        ((coord.x - 1)..=(coord.x + 1))
            .cartesian_product(
                ((coord.y - 1)..=(coord.y + 1)).cartesian_product((coord.z - 1)..=(coord.z + 1)),
            )
            .filter_map(move |(x, (y, z))| {
                let candidate = Coord { x, y, z };
                if candidate != coord {
                    Some(self.current_state_for_coord(candidate))
                } else {
                    None
                }
            })
    }

    fn next_bounds(&self) -> (Coord, Coord) {
        (
            Coord {
                x: self.min_bound.x - 1,
                y: self.min_bound.y - 1,
                z: self.min_bound.z - 1,
            },
            Coord {
                x: self.max_bound.x + 1,
                y: self.max_bound.y + 1,
                z: self.max_bound.z + 1,
            },
        )
    }

    fn next_coords(&self) -> impl Iterator<Item = Coord> {
        let (next_min, next_max) = self.next_bounds();

        (next_min.x..=next_max.x)
            .cartesian_product((next_min.y..=next_max.y).cartesian_product(next_min.z..=next_max.z))
            .map(|(x, (y, z))| Coord { x, y, z })
    }

    fn next(&self) -> ConwayCubes {
        let mut next_data: HashMap<Coord, CubeState> = HashMap::new();
        let (next_min, next_max) = self.next_bounds();

        for coord in self.next_coords() {
            let neighbouring_states = self.neighbouring_states(coord);

            let counts: Counter<CubeState> = neighbouring_states.collect();

            let active_count = counts.get(&CubeState::Active).unwrap_or(&0);

            let current = self.current_state_for_coord(coord);

            let next_state = match (current, *active_count) {
                (CubeState::Active, 2 | 3) => CubeState::Active,
                (CubeState::Inactive, 3) => CubeState::Active,
                _ => CubeState::Inactive,
            };

            next_data.insert(coord, next_state);
        }

        ConwayCubes {
            data: next_data,
            min_bound: next_min,
            max_bound: next_max,
        }
    }

    fn active_count(&self) -> usize {
        self.next_coords()
            .map(|coord| {
                if (self.current_state_for_coord(coord) == CubeState::Active) {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}
