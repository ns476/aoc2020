use std::collections::HashSet;
use std::str::FromStr;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/eight/input")?;

    let instructions: Vec<Instruction> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse().unwrap())
        .collect();

    let mut computer = Computer::for_instructions(instructions);

    while !computer.has_executed_next_instruction() {
        computer.step();
    }

    dbg!(computer.acc);

    Ok(())
}

#[derive(Debug)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elems: Vec<_> = s.split(' ').collect();

        let instr = elems.get(0).ok_or("No instruction!")?;
        let idx_str = elems.get(1).ok_or("Missing index!")?;
        let idx = idx_str.parse().map_err(|_| "Couldn't parse index!")?;

        match *instr {
            "acc" => Ok(Instruction::Acc(idx)),
            "jmp" => Ok(Instruction::Jmp(idx)),
            "nop" => Ok(Instruction::Nop(idx)),
            _ => Err("Unknown instruction"),
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
            instructions,
            acc: 0,
            pc: 0,
            executed_instructions: HashSet::new(),
        }
    }

    fn step(&mut self) {
        self.executed_instructions.insert(self.pc);
        match self.instructions[self.pc] {
            Instruction::Acc(x) => self.acc += x,
            Instruction::Jmp(x) => self.pc = ((self.pc as i32) + (x - 1)) as usize,
            Instruction::Nop(_) => (),
        }

        self.pc += 1;
    }

    fn has_executed_next_instruction(&self) -> bool {
        self.executed_instructions.contains(&self.pc)
    }
}
