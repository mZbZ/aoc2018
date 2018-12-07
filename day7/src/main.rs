#![feature(nll)]
#![feature(vec_remove_item)]

extern crate regex;

use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    let _test_input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    assert_eq!(part1(_test_input), "CABDFE");
    println!("Part1 {:?}", part1(input));

    assert_eq!(part2(_test_input, 64, 1), 15);
    println!("Part2 {:?}", part2(input, 4, 4));
}

fn part2(input: &str, delay: u8, helpers: u8) -> usize {
    let mut nodes: HashMap<char, (Vec<char>, u8)> = HashMap::new();
    input.lines().for_each(|x| {
        let new_char = x.chars().nth(5).unwrap();
        let entry = nodes
            .entry(new_char)
            .or_insert((vec![], new_char as u8 - delay));
        entry.0.push(x.chars().nth(36).unwrap());
    });
    println!("{:?}", nodes);
    // let prosp_nodes = find_first_node(&nodes);
    // traverse(nodes, prosp_nodes);
    unimplemented!();
}

fn part1(input: &str) -> String {
    let mut nodes: HashMap<char, (Vec<char>, bool)> = HashMap::new();
    input.lines().for_each(|x| {
        let entry = nodes
            .entry(x.chars().nth(5).unwrap())
            .or_insert((vec![], false));
        entry.0.push(x.chars().nth(36).unwrap());
    });
    let prosp_nodes = find_first_node(&nodes);
    traverse(nodes, prosp_nodes)
}

fn find_first_node(nodes: &HashMap<char, (Vec<char>, bool)>) -> Vec<char> {
    let mut first_nodes = nodes.keys().fold(vec![], |mut acc, x| {
        if nodes.values().map(|x| &x.0).flatten().all(|y| y != x) {
            acc.push(*x)
        }
        acc
    });
    first_nodes.sort_unstable();
    first_nodes
}

fn traverse(mut nodes: HashMap<char, (Vec<char>, bool)>, mut prosp_nodes: Vec<char>) -> String {
    let mut result = String::new();
    let mut curr_char = prosp_nodes[0];
    loop {
        result += curr_char.to_string().as_str();
        nodes.entry(curr_char).and_modify(|c| c.1 = true);
        prosp_nodes.remove_item(&curr_char);
        if let Some(next_char) = find_next(&mut nodes, &mut prosp_nodes, curr_char) {
            curr_char = next_char;
        } else {
            break;
        }
    }
    result
}

fn find_next(
    nodes: &mut HashMap<char, (Vec<char>, bool)>,
    prosp_nodes: &mut Vec<char>,
    curr_char: char,
) -> Option<char> {
    prosp_nodes.append(&mut nodes.get(&curr_char).unwrap_or(&(vec![], false)).0.clone());
    prosp_nodes.sort_unstable();
    prosp_nodes.dedup();
    for next_alpha in prosp_nodes {
        if nodes.values().all(|aa| !aa.0.contains(&next_alpha) || aa.1) {
            return Some(*next_alpha);
        }
    }
    None
}
