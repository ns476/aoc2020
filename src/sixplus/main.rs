extern crate lazy_static;
extern crate regex;

use std::collections::HashSet;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/sixplus/input")?;


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
    dbg!(group);
    let mut people = group.split('\n').peekable();

    let first = people
        .peek()
        .map(|x| x.chars().collect::<HashSet<char>>())
        .unwrap_or_default();

    let answered_questions = people.fold(first, |cur, person| {
        person
            .chars()
            .collect::<HashSet<char>>()
            .intersection(&cur)
            .cloned()
            .collect::<HashSet<char>>()
    });

    dbg!(&answered_questions);

    answered_questions.len()
}
