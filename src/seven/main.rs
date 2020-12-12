#[macro_use]
extern crate lazy_static;
extern crate regex;

extern crate petgraph;

use petgraph::dot::Dot;
use petgraph::graph::NodeIndex;
use petgraph::prelude::DiGraph;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/seven/testinput")?;

    lazy_static! {
        static ref PASSPORT_SEP: Regex = Regex::new("\n\n").unwrap();
    }

    let bag_specs = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| parse_bag(&x))
        .map(|x| {
            dbg!(&x);
            x
        })
        .collect();

    let bag_graph = build_specs_graph(bag_specs);

    println!("{:?}", Dot::with_config(&bag_graph.contained_by_graph, &[]));

    let containing_bags = find_containing_bags(&bag_graph, "shiny gold");

    dbg!(&containing_bags);
    dbg!(containing_bags.len());

    Ok(())
}

struct BagGraph {
    containing_graph: DiGraph<String, f32>,
    contained_by_graph: DiGraph<String, f32>,
    nodes_by_name: HashMap<String, NodeIndex>,
    names_by_node: HashMap<NodeIndex, String>,
}

#[derive(Debug)]
struct BagSpec {
    name: String,
    bags_inside: HashMap<String, f32>,
}

fn parse_bag(bag_description: &str) -> BagSpec {
    let top_split = bag_description.split(" bags contain ").collect::<Vec<_>>();

    let bag_name = String::from(top_split[0]);

    // 2 shiny gold bags, 9 faded blue bags.
    let bags = top_split[1];

    lazy_static! {
        static ref BAG_RE: Regex = Regex::new(r"(\d+) (.+?) bag(s)?").unwrap();
    }

    let bags_inside = BAG_RE
        .captures_iter(bags)
        .map(|x| {
            (
                String::from(x.get(2).unwrap().as_str()),
                x.get(1).unwrap().as_str().parse::<f32>().unwrap(),
            )
        })
        .collect::<HashMap<String, f32>>();

    BagSpec {
        name: bag_name,
        bags_inside: bags_inside,
    }
}

fn build_specs_graph(bag_specs: Vec<BagSpec>) -> BagGraph {
    let mut graph: DiGraph<String, f32> = DiGraph::default();
    let mut nodes_by_name: HashMap<String, NodeIndex> = HashMap::new();
    let mut names_by_node: HashMap<NodeIndex, String> = HashMap::new();

    bag_specs.iter().for_each(|spec| {
        let node = graph.add_node(String::from(&spec.name));
        nodes_by_name.insert(String::from(&spec.name), node);
        names_by_node.insert(node, String::from(&spec.name));
    });

    bag_specs.iter().for_each(|spec| {
        let parent_node = nodes_by_name.get(&spec.name).unwrap();
        spec.bags_inside.iter().for_each(|(child_name, count)| {
            let child_node = nodes_by_name.get(child_name).unwrap();

            graph.add_edge(*parent_node, *child_node, *count);
        });
    });

    BagGraph {
        containing_graph: graph.clone(),
        contained_by_graph: {
            graph.reverse();
            graph
        },
        nodes_by_name: nodes_by_name,
        names_by_node: names_by_node,
    }
}

fn find_containing_bags(bag_graph: &BagGraph, bag_name: &str) -> HashSet<String> {
    let bellman_ford_results = petgraph::algo::bellman_ford(
        &bag_graph.contained_by_graph,
        *bag_graph.nodes_by_name.get(bag_name).unwrap(),
    );

    let (_, nodes) = bellman_ford_results.unwrap();

    let mut reachable_node_names = HashSet::default();

    for (idx, predecessor_if_reached) in nodes.iter().enumerate() {
        match predecessor_if_reached {
            Some(_) => {
                reachable_node_names.insert(String::from(
                    bag_graph.names_by_node.get(&NodeIndex::new(idx)).unwrap(),
                ));
            }
            _ => (),
        }
    }

    reachable_node_names
}
