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

    assert_eq!(part2(_test_input, 64, 2), 15);
    println!("Part2 {:?}", part2(input, 4, 5));
}

fn part2(input: &str, delay: u8, helpers: usize) -> usize {
    let mut nodes: HashMap<char, (Vec<char>, u8)> = HashMap::new();
    input.lines().for_each(|x| {
        let new_char = x.chars().nth(5).unwrap();
        let entry = nodes
            .entry(new_char)
            .or_insert((vec![], new_char as u8 - delay));
        entry.0.push(x.chars().nth(36).unwrap());
    });

    let base_nodes = find_first_node(&nodes);
    let mut working_nodes = base_nodes.into_iter().take(helpers).collect();
    let mut prosp_nodes = base_nodes.into_iter().skip(helpers).collect();
    work(nodes, working_nodes, prosp_nodes, helpers)
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

fn find_first_node<T>(nodes: &HashMap<char, (Vec<char>, T)>) -> Vec<char> {
    let mut first_nodes = nodes.keys().fold(vec![], |mut acc, x| {
        if nodes.values().map(|x| &x.0).flatten().all(|y| y != x) {
            acc.push(*x)
        }
        acc
    });
    first_nodes.sort_unstable();
    first_nodes
}

fn work(
    mut nodes: HashMap<char, (Vec<char>, u8)>,
    mut working_nodes: Vec<char>,
    mut prosp_nodes: Vec<char>,
    helpers: usize,
) -> usize {
    let mut time = 0;
    loop {
        for working_node in working_nodes {
            nodes.entry(working_node).and_modify(|c| c.1 -= 1);
        }
        if let Some(new_workers) =
            find_next_workers(&mut nodes, &mut working_nodes, &mut prosp_nodes)
        {
            for new_worker in new_workers {
                if working_nodes.len() < helpers {
                    working_nodes.push(new_worker);
                } else {
                    prosp_nodes.push(new_worker);
                }
            }
        } else {
            break;
        }
        time += 1
    }
    time
}

fn find_next_workers(
    nodes: &mut HashMap<char, (Vec<char>, u8)>,
    working_nodes: &mut Vec<char>,
    prosp_nodes: &mut Vec<char>,
) -> Option<Vec<char>> {
    prosp_nodes.append(&mut nodes.get(&curr_char).unwrap_or(&(vec![], 0)).0.clone());
    prosp_nodes.sort_unstable();
    prosp_nodes.dedup();
    let mut results = None;
    for next_alpha in working_nodes {
        if let Some(new_worker) = nodes
            .values()
            .find(|aa| aa.0.contains(&next_alpha) || aa.1 == 0)
        {
            if results.is_none() {
                results = Some(vec![]);
            }
            results.unwrap().push(next_alpha.clone())
        }
    }
    results
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
