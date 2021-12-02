use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/twelve/input")?;

    let elems: Vec<Instruction> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse().unwrap())
        .collect();

    let mut state = State::new();

    for elem in elems {
        dbg!(&elem);
        state.apply_instruction(elem);
        dbg!(&state);
    }

    dbg!(state.manhattan());

    Ok(())
}

#[derive(Debug)]
struct State {
    direction: Direction,
    x: i64,
    y: i64,
}

impl State {
    fn new() -> State {
        State {
            direction: Direction::East,
            x: 0,
            y: 0,
        }
    }

    fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction.code {
            Code::Direction(dir) => dir.apply(self, instruction.count),
            Code::Left => self.direction = self.direction.rotate(-(instruction.count as i64)),
            Code::Right => self.direction = self.direction.rotate(instruction.count as i64),
            Code::Forward => {
                let dir: Direction = self.direction;
                dir.apply(self, instruction.count)
            }
        }
    }

    fn manhattan(&self) -> i64 {
        i64::abs(self.x) + i64::abs(self.y)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(&self, degrees: i64) -> Direction {
        let mut dirs = vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];

        while dirs.get(0) != Some(self) {
            dirs.rotate_right(1);
        }

        match degrees / 90 {
            x if x >= 0 => dirs.rotate_left(x as usize),
            x => dirs.rotate_right(-x as usize),
        }

        *(dirs.get(0).unwrap())
    }

    fn apply(&self, state: &mut State, steps: usize) {
        match self {
            Direction::North => state.y += steps as i64,
            Direction::East => state.x += steps as i64,
            Direction::West => state.x -= steps as i64,
            Direction::South => state.y -= steps as i64,
        }
    }
}

#[derive(Debug)]
enum Code {
    Direction(Direction),
    Left,
    Right,
    Forward,
}

#[derive(Debug)]
struct Instruction {
    code: Code,
    count: usize,
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(x: &str) -> Result<Instruction, Self::Err> {
        let code = x
            .chars()
            .next()
            .and_then(|x| match x {
                'N' => Some(Code::Direction(Direction::North)),
                'S' => Some(Code::Direction(Direction::South)),
                'E' => Some(Code::Direction(Direction::East)),
                'W' => Some(Code::Direction(Direction::West)),
                'L' => Some(Code::Left),
                'R' => Some(Code::Right),
                'F' => Some(Code::Forward),
                _ => None,
            })
            .ok_or(format!("Invalid code in instruction: {}", x))?;

        let count = &x[1..x.len()]
            .parse::<usize>()
            .map_err(|_| format!("Invalid digits {}", x))?;

        Ok(Instruction {
            code,
            count: *count,
        })
    }
}
