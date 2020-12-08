extern crate regex;
extern crate aoc_2020;

use aoc_2020::util;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn main() {
    let bags = parse_bag_graph("inputs/d7");
    println!("Found {} unique bag rules", bags.len());

    let mut parents = HashSet::new();
    let target = "shiny gold";
    for root in bags.keys() {
        if root != target && is_reachable(&bags, root, target) {
            parents.insert(root);
        }
    }
    println!("Number of parents {}", parents.len());

    println!("Number of bags in one {} bag = {}", target, dumb_graph_sum(&bags, target));
}

fn parse_bag_graph(filepath: &str)  -> HashMap<String, BagNode> {
    let mut bags: HashMap<String, BagNode> = HashMap::new();

    let start_re = Regex::new(r"^\s*(\w+ \w+) bags contain").unwrap();
    let bag_re = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
    let no_other_re = Regex::new(r"no other bags").unwrap();

    for line in util::read_lines(filepath).unwrap() {
        let line_str = line.unwrap();
        assert!(start_re.is_match(&line_str));
        let node_name = start_re.captures(&line_str).unwrap().get(1).unwrap().as_str();

        // each line defines rules for a unique bag kind
        assert!(!bags.contains_key(node_name));

        if bag_re.is_match(&line_str) {
            let mut childs = Vec::new();
            for child_cap in bag_re.captures_iter(&line_str) {
                let n = child_cap[1].parse::<usize>().unwrap();
                let typ = &child_cap[2];
                childs.push(BagChild { num: n, typ: typ.to_string() });
            }
            bags.insert(node_name.to_string(), BagNode(childs));
        } else if no_other_re.is_match(&line_str) {
            bags.insert(node_name.to_string(), BagNode(Vec::new()));
        } else {
            panic!("unexpected input line {}", &line_str);
        }
    }
    bags
}

fn is_reachable(graph: &HashMap<String, BagNode>, src: &str, dest: &str) -> bool {
    let mut seen: HashSet<String> = HashSet::new();
    let mut frontier: HashSet<String> = HashSet::new();
    frontier.insert(src.to_string());

    while !frontier.is_empty() {
        let mut new_frontier = HashSet::new();
        for cur in frontier.iter() {
            if cur == dest {
                return true;
            }
            let child_bags = graph.get(cur)
                .expect("oops, graph has dangling edge");
            seen.insert(cur.to_string());
            for child in child_bags.0.iter() {
                if !seen.contains(&child.typ) {
                    new_frontier.insert(child.typ.clone());
                }
            }
        }
        frontier = new_frontier;
    }
    false
}

fn dumb_graph_sum(graph: &HashMap<String, BagNode>, root: &str) -> usize {
    let child_bags = graph.get(root)
        .expect("oops, graph has dangling edge");
    if child_bags.0.is_empty() {
        0
    } else {
        let mut sum = 0;
        for child in child_bags.0.iter() {
            sum += child.num * (1 + dumb_graph_sum(graph, &child.typ));
        }
        sum
    }
}

// TODO
//   - replace HashMap<String, BagNode> with a proper graph
//   - intern bag names

#[derive(Debug)]
pub struct BagNode(Vec<BagChild>);

#[derive(Debug)]
pub struct BagChild {
    num: usize,
    typ: String,
}

#[cfg(test)]
mod test_d7 {
    use super::*;

    const EXAMPLES_RULES: &'static str =
        "light red bags contain 1 bright white bag, 2 muted yellow bags.
         dark orange bags contain 3 bright white bags, 4 muted yellow bags.
         bright white bags contain 1 shiny gold bag.
         muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
         shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
         dark olive bags contain 3 faded blue bags, 4 dotted black bags.
         vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
         faded blue bags contain no other bags.
         dotted black bags contain no other bags.";

    #[test]
    fn test_example_rules_re() {
        let re = Regex::new(r"^\s*\w+ \w+ bags contain").unwrap();
        for line in EXAMPLES_RULES.lines() {
            assert!(re.is_match(line));
        }
        let re = Regex::new(r"(\d+) (\w+) (\w+) bags?").unwrap();
        for line in EXAMPLES_RULES.lines() {
            for grp in re.captures_iter(line) {
                assert!(grp[1].parse::<usize>().is_ok());
            }
        }
    }

    #[test]
    fn test_reachability() {
        let bags = parse_bag_graph("inputs/d7_test");
        assert_eq!(bags.len(), 9);

        let mut parents = HashSet::new();
        let target = "shiny gold";
        for root in bags.keys() {
            if root != target && is_reachable(&bags, root, target) {
                parents.insert(root);
                println!("{} -> {}", root, target);
            }
        }
        assert_eq!(parents.len(), 4);
    }

    #[test]
    fn test_dumb_graph_sum() {
        let bags = parse_bag_graph("inputs/d7_test");
        assert_eq!(dumb_graph_sum(&bags, "shiny gold"), 32);
    }
}
