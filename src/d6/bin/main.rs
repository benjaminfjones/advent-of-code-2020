extern crate aoc_2020;

use aoc_2020::util;
use std::collections::HashSet;

pub fn main() {
    let mut results: Vec<HashSet<char>> = Vec::new();
    let all_set: HashSet<char> = ('a'..='z').collect();
    let mut set = all_set.clone();
    for line in util::read_lines("inputs/d6").unwrap() {
        let line = line.unwrap();
        let line_set: HashSet<char> = line.trim().chars().collect();
        if line_set.len() == 0 {
            results.push(set.clone());
            set = all_set.clone();
            continue;
        }

        let intersect = line_set.into_iter().filter(|c| set.contains(c)).collect();
        set = intersect;
    }
    if !set.is_empty() {
        results.push(set);
    }

    let mut sum = 0;
    for grp in results.iter() {
        println!("grp = {:?}", grp);
        sum += grp.len();
    }
    println!("sum = {}", sum);
}
