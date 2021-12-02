#[macro_use]
extern crate lazy_static;
extern crate counter;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use counter::Counter;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/sixteenplus/input")?;
    let lines = BufReader::new(file).lines().map(|x| x.unwrap());

    lazy_static! {
        static ref BOUND_RE: Regex = Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        static ref TICKET_RE: Regex = Regex::new(r"^([0-9,]+)+$").unwrap();
    }

    let mut bounds: HashMap<String, ValidRanges> = HashMap::new();
    let mut tickets: Vec<Vec<u64>> = Vec::new();

    for line in lines {
        if let Some(capt) = BOUND_RE.captures(&line) {
            bounds.insert(
                capt.get(1).unwrap().as_str().to_string(),
                ValidRanges {
                    ranges: vec![
                        capt.get(2).unwrap().as_str().parse::<u64>().unwrap()
                            ..=capt.get(3).unwrap().as_str().parse::<u64>().unwrap(),
                        capt.get(4).unwrap().as_str().parse::<u64>().unwrap()
                            ..=capt.get(5).unwrap().as_str().parse::<u64>().unwrap(),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                },
            );
        }

        if TICKET_RE.find(&line).is_some() {
            tickets.push(line.split(',').map(|x| x.parse::<u64>().unwrap()).collect());
        }
    }

    let mut valid_tickets = tickets.iter().filter(|fields| {
        fields.iter().all(|&x| {
            bounds
                .values()
                .any(|valid_range| valid_range.any_is_valid(x))
        })
    });

    // Skip our ticket
    let our_ticket = valid_tickets.next().unwrap();

    let other_tickets: Vec<_> = valid_tickets.collect();

    let assignments = find_bound_assignment(other_tickets, bounds.clone());

    let mut product = 1;

    dbg!(&assignments);

    for (name, index) in assignments {
        if name.starts_with("departure") {
            product *= our_ticket.get(index).unwrap();
        }
    }

    println!("product: {}", product);

    Ok(())
}

fn find_bound_assignment(
    other_tickets: Vec<&Vec<u64>>,
    mut bounds: HashMap<String, ValidRanges>,
) -> HashMap<String, usize> {
    #[derive(Eq, PartialEq, Debug)]
    enum AssignmentType<T> {
        NoAssignment,
        OneAssignment(T),
        ManyAssignments,
    }

    let field_count = other_tickets.get(0).unwrap().len();
    let mut assignments: HashMap<String, usize> = HashMap::new();

    while !&bounds.is_empty() {
        let assigned_cols: HashSet<_> = assignments.values().collect();
        let mut possible_assignment_bound: HashMap<String, AssignmentType<usize>> = HashMap::new();
        let mut possible_assignment_col: HashMap<usize, AssignmentType<String>> = HashMap::new();

        for (bound_name, bound_condition) in bounds.clone() {
            for i in 0..=(field_count - 1) {
                if !assigned_cols.contains(&i)
                    && other_tickets.iter().all(|t| {
                        let field = t.get(i).unwrap();
                        bound_condition.any_is_valid(*field)
                    })
                {
                    dbg!((i, &bound_name));
                    possible_assignment_bound
                        .entry(bound_name.clone())
                        .or_insert(AssignmentType::NoAssignment);

                    possible_assignment_col
                        .entry(i)
                        .or_insert(AssignmentType::NoAssignment);

                    possible_assignment_bound
                        .entry(bound_name.clone())
                        .and_modify(|e| {
                            *e = match *e {
                                AssignmentType::NoAssignment => AssignmentType::OneAssignment(i),
                                AssignmentType::OneAssignment(_) => AssignmentType::ManyAssignments,
                                AssignmentType::ManyAssignments => AssignmentType::ManyAssignments,
                            }
                        });

                    possible_assignment_col.entry(i).and_modify(|e| {
                        *e = match *e {
                            AssignmentType::NoAssignment => {
                                AssignmentType::OneAssignment(bound_name.clone())
                            }
                            AssignmentType::OneAssignment(_) => AssignmentType::ManyAssignments,
                            AssignmentType::ManyAssignments => AssignmentType::ManyAssignments,
                        }
                    });
                }
            }
        }

        dbg!(&possible_assignment_bound);
        dbg!(&possible_assignment_col);

        if (possible_assignment_bound.is_empty() && possible_assignment_col.is_empty()) {
            panic!("EMPTY EMPTY")
        }

        for (bound_name, assignment) in possible_assignment_bound {
            match assignment {
                AssignmentType::NoAssignment => {}
                AssignmentType::OneAssignment(x) => {
                    dbg!((&bound_name, &x));
                    assignments.insert(bound_name.clone(), x);
                    bounds.remove(&bound_name.clone());
                }
                AssignmentType::ManyAssignments => {}
            }
        }

        for (col, assignment) in possible_assignment_col {
            match assignment {
                AssignmentType::NoAssignment => {}
                AssignmentType::OneAssignment(bound_name) => {
                    dbg!((&bound_name, &col));
                    assignments.insert(bound_name.clone(), col);
                    bounds.remove(&bound_name.clone());
                }
                AssignmentType::ManyAssignments => {}
            }
        }
    }

    assignments
}

#[derive(Debug, Clone)]
struct ValidRanges {
    ranges: HashSet<std::ops::RangeInclusive<u64>>,
}

impl ValidRanges {
    fn any_is_valid(&self, x: u64) -> bool {
        self.ranges.iter().any(|range| range.contains(&x))
    }
}
