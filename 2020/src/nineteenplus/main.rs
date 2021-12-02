extern crate counter;
extern crate regex;

#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

use itertools::Intersperse;
use itertools::Itertools;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let context: String = std::fs::read_to_string("src/nineteenplus/input")?;
    let (rule_part, test_part) = context.split("\n\n").collect_tuple().unwrap();

    let rules: Vec<String> = rule_part.split("\n").map(|x| x.to_string()).collect();

    let rule_map = to_rule_map(rules);
    let re = to_regexp(&rule_map);

    let success_count: usize = test_part
        .split("\n")
        .map(|x| if re.is_match(x) { 1 } else { 0 })
        .sum();

    dbg!(success_count);

    Ok(())
}

#[derive(Debug, Clone)]
enum Rule {
    Ref(usize),
    Literal(char),
    Sequence(Vec<Rule>),
    Alternative(Vec<Rule>),
}
fn to_regexp(rule_map: &HashMap<usize, Rule>) -> Regex {
    fn to_regexp_basic(rule_map: &HashMap<usize, Rule>, rule: &Rule) -> String {
        match rule {
            Rule::Ref(x) => to_regexp_index(rule_map, *x),
            Rule::Literal(c) => format!("({})", c),
            Rule::Sequence(rules) => {
                format!(
                    "({})",
                    rules
                        .clone()
                        .iter()
                        .map(|r| to_regexp_basic(rule_map, &r))
                        .join("")
                )
            }
            Rule::Alternative(rules) => {
                format!(
                    "({})",
                    rules
                        .clone()
                        .iter()
                        .map(|r| to_regexp_basic(rule_map, &r))
                        .join("|")
                )
            }
        }
    }
    fn to_regexp_index(rule_map: &HashMap<usize, Rule>, index: usize) -> String {
        match index {
            8 => {
                let fourty_two = to_regexp_index(rule_map, 42);

                format!("(({})+)", fourty_two)
            }
            11 => {
                let fourty_two = to_regexp_index(rule_map, 42);
                let thirty_one = to_regexp_index(rule_map, 31);
                format!(
                    "({})",
                    (1..5)
                        .map(|x| {
                            format!("({})({})", fourty_two.repeat(x), thirty_one.repeat(x))
                        })
                        .join("|")
                )
            }
            _ => {
                let rule = rule_map.get(&index).unwrap();
                to_regexp_basic(rule_map, rule)
            }
        }
    }

    let regexp_str = format!("^{}$", to_regexp_index(&rule_map, 0));

    dbg!(&regexp_str);

    Regex::new(&regexp_str).unwrap()
}

fn to_rule_map(rules: Vec<String>) -> HashMap<usize, Rule> {
    fn parse_leaf_rule(leaf: &str) -> Rule {
        let num_ref: Option<usize> = leaf.parse().ok();

        match num_ref {
            Some(x) => Rule::Ref(x),
            _ => Rule::Literal(leaf.chars().nth(1).unwrap()),
        }
    }

    fn parse_alternativeless_rule(alternativeless: &str) -> Rule {
        let sequence: Vec<&str> = alternativeless.split_whitespace().collect();

        if sequence.len() == 1 {
            parse_leaf_rule(sequence.get(0).unwrap())
        } else {
            Rule::Sequence(sequence.iter().cloned().map(parse_leaf_rule).collect())
        }
    }

    fn parse_complex_rule(str: &String) -> (usize, Rule) {
        let (num_str, rule_part) = str.split(": ").collect_tuple().unwrap();
        let num = num_str.parse().unwrap();

        let alternatives: Vec<_> = rule_part.split(" | ").collect();

        (
            num,
            if alternatives.len() == 1 {
                parse_alternativeless_rule(alternatives.get(0).unwrap())
            } else {
                Rule::Alternative(
                    alternatives
                        .iter()
                        .cloned()
                        .map(parse_alternativeless_rule)
                        .collect(),
                )
            },
        )
    }

    rules.iter().map(parse_complex_rule).collect()
}
