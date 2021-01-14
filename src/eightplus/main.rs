#[macro_use]
extern crate lazy_static;
extern crate regex;

extern crate petgraph;

use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/eight/input")?;

    lazy_static! {
        static ref PASSPORT_SEP: Regex = Regex::new("\n\n").unwrap();
    }

    let instructions: Vec<Instruction> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse().unwrap())
        .collect();

    for i in 0..instructions.len() {
        match uncorrupt(&instructions, i) {
            Some(uncorrupted) => {
                let mut computer = Computer::for_instructions(uncorrupted);

                if run_until_end(&mut computer) {
                    dbg!(computer.acc);
                    break;
                }
            }
            None => (),
        }
    }

    Ok(())
}

fn uncorrupt(corrupted_instructions: &Vec<Instruction>, index: usize) -> Option<Vec<Instruction>> {
    match corrupted_instructions[index] {
        Instruction::Nop(_) | Instruction::Jmp(_) => {
            let mut fixed = corrupted_instructions.clone();
            fixed[index] = match fixed[index] {
                Instruction::Nop(x) => Instruction::Jmp(x),
                Instruction::Jmp(x) => Instruction::Nop(x),
                _ => panic!("Unexpected instruction!"),
            };
            Some(fixed)
        }
        _ => None,
    }
}

fn run_until_end(computer: &mut Computer) -> bool {
    let mut finished_successfully = false;
    loop {
        if computer.has_finished() {
            finished_successfully = true;
            break;
        }

        if computer.has_executed_next_instruction() {
            break;
        }

        computer.step();
    }

    finished_successfully
}

#[derive(Debug, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elems: Vec<_> = s.split(" ").collect();

        let instr = elems.get(0).ok_or("No instruction!")?;
        let idx_str = elems.get(1).ok_or("Missing index!")?;
        let idx = idx_str.parse().map_err(|_| "Couldn't parse index!")?;

        match instr {
            &"acc" => Ok(Instruction::Acc(idx)),
            &"jmp" => Ok(Instruction::Jmp(idx)),
            &"nop" => Ok(Instruction::Nop(idx)),
            &_ => Err("Unknown instruction"),
        }
    }
}

struct Computer {
    instructions: Vec<Instruction>,
    acc: i32,
    pc: usize,
    executed_instructions: HashSet<usize>,
}

impl Computer {
    fn for_instructions(instructions: Vec<Instruction>) -> Computer {
        Computer {
            instructions: instructions,
            acc: 0,
            pc: 0,
            executed_instructions: HashSet::new(),
        }
    }

    fn step(&mut self) {
        self.executed_instructions.insert(self.pc);
        match self.instructions[self.pc] {
            Instruction::Acc(x) => {
                self.acc += x;
                self.pc += 1;
            }
            Instruction::Jmp(x) => self.pc = ((self.pc as i32) + x) as usize,
            Instruction::Nop(_) => self.pc += 1,
        }
    }

    fn has_executed_next_instruction(&self) -> bool {
        return self.executed_instructions.contains(&self.pc);
    }

    fn has_finished(&self) -> bool {
        return self.instructions.len() <= self.pc;
    }
}
