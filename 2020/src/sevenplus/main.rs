#[macro_use]
extern crate lazy_static;
extern crate regex;

use cached::proc_macro::cached;
use cached::stores::UnboundCache;

extern crate petgraph;

use petgraph::graph::NodeIndex;
use petgraph::prelude::DiGraph;
use petgraph::visit::EdgeRef;
use petgraph::Outgoing;
use regex::Regex;
use std::collections::HashMap;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/seven/input")?;

    let bag_specs = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| parse_bag(&x))
        .collect();

    let bag_graph = build_specs_graph(bag_specs);

    dbg!(find_bags_inside_shiny_gold(&bag_graph));

    Ok(())
}

#[allow(dead_code)]
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
        bags_inside,
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
        nodes_by_name,
        names_by_node,
    }
}

fn find_bags_inside_shiny_gold(bag_graph: &BagGraph) -> usize {
    #[cached(
        type = "UnboundCache<String, usize>",
        create = "{ UnboundCache::new() }",
        convert = r#"{ String::from(name) }"#
    )]
    fn lookup(bag_graph: &BagGraph, name: &str) -> usize {
        let node = bag_graph.nodes_by_name.get(name).unwrap();

        dbg!(name);
        let bags_inside = bag_graph
            .containing_graph
            .edges_directed(*node, Outgoing)
            .map(|edge| {
                let target_node_name = bag_graph.names_by_node.get(&edge.target()).unwrap();

                let bags_inside = lookup(&bag_graph, target_node_name);
                dbg!((name, target_node_name, edge.weight()));
                (*edge.weight() as usize) * (bags_inside + 1)
            })
            .sum();

        bags_inside
    }

    lookup(&bag_graph, "shiny gold")
}
