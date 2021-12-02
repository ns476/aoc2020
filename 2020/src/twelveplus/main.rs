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
    wpt_x: i64,
    wpt_y: i64,
    x: i64,
    y: i64,
}

impl State {
    fn new() -> State {
        State {
            wpt_x: 10,
            wpt_y: 1,
            x: 0,
            y: 0,
        }
    }

    fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction.code {
            Code::Direction(dir) => dir.apply(self, instruction.count),
            Code::Left => self.rotate_wpt(instruction.count as i64),
            Code::Right => self.rotate_wpt(-(instruction.count as i64)),
            Code::Forward => {
                self.x += self.wpt_x * (instruction.count as i64);
                self.y += self.wpt_y * (instruction.count as i64);
            }
        }
    }

    fn rotate_wpt(&mut self, degrees: i64) {
        for _ in 0..(i64::abs(degrees / 90)) {
            let (new_x, new_y) = if degrees > 0 {
                (-self.wpt_y, self.wpt_x)
            } else {
                (self.wpt_y, -self.wpt_x)
            };

            self.wpt_x = new_x;
            self.wpt_y = new_y;
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
    fn apply(&self, state: &mut State, steps: usize) {
        match self {
            Direction::North => state.wpt_y += steps as i64,
            Direction::East => state.wpt_x += steps as i64,
            Direction::West => state.wpt_x -= steps as i64,
            Direction::South => state.wpt_y -= steps as i64,
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
