use std::collections::HashSet;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/six/input")?;

    let file: String = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    let groups = file.split("\n\n").collect::<Vec<_>>();

    let questions_answered = groups
        .iter()
        .map(|x| count_answered_questions(x))
        .sum::<usize>();

    println!("{}", questions_answered);

    Ok(())
}

fn count_answered_questions(group: &str) -> usize {
    group
        .replace('\n', "")
        .chars()
        .collect::<HashSet<_>>()
        .len()
}
