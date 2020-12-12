#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashSet;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/sixplus/input")?;

    lazy_static! {
        static ref GROUP_SEP: Regex = Regex::new("\n\n").unwrap();
    }

    let file: String = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    let groups = GROUP_SEP.split(&file).collect::<Vec<_>>();

    let questions_answered = groups
        .iter()
        .map(|x| count_answered_questions(x))
        .sum::<usize>();

    println!("{}", questions_answered);

    Ok(())
}

fn count_answered_questions(group: &str) -> usize {
    dbg!(group);
    let mut people = group.split("\n").peekable();

    let first = people.peek().map(|x| x.chars().collect::<HashSet<char>>()).unwrap_or(HashSet::new());

    let answered_questions = people
        .fold(first, |cur, person| {
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
