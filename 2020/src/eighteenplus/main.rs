extern crate counter;
extern crate pest;
extern crate regex;
#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate lazy_static;

use std::error::Error;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;

#[derive(Parser)]
#[grammar = "eighteen/calc.pest"]
struct CalcParser;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(multiply, Left)
                | Operator::new(divide, Left)
                | Operator::new(power, Left),
            Operator::new(add, Left) | Operator::new(subtract, Left),
        ])
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("src/eighteenplus/input")?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let sum: i64 = lines
        .map(|res| {
            let line = res.unwrap();

            let expr = CalcParser::parse(Rule::calculation, &line).unwrap();

            dbg!(&expr);

            eval(expr)
        })
        .sum();

    dbg!(sum);

    Ok(())
}

fn eval(expression: Pairs<Rule>) -> i64 {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => pair.as_str().parse::<i64>().unwrap(),
            Rule::expr => eval(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: i64, op: Pair<Rule>, rhs: i64| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            Rule::power => lhs.pow(rhs.try_into().unwrap()),
            _ => unreachable!(),
        },
    )
}
