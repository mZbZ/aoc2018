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
        let first_char = x.chars().nth(5).unwrap();
        let second_char = x.chars().nth(36).unwrap();

        let first = nodes
            .entry(first_char)
            .or_insert((vec![], first_char as u8 - delay));
        first.0.push(second_char);

        nodes
            .entry(second_char)
            .or_insert((vec![], second_char as u8 - delay));
    });
    println!("{:?}", nodes);

    let base_nodes = find_first_node(&nodes);
    let working_nodes = base_nodes.clone().into_iter().take(helpers).fold(
        Vec::with_capacity(helpers),
        |mut acc, x| {
            acc.push(x);
            acc
        },
    );
    let prosp_nodes = base_nodes.into_iter().skip(helpers).collect();
    work(nodes, working_nodes, prosp_nodes)
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
) -> usize {
    let mut time = 0;
    loop {
        println!(
            "Time {:?} Working Nodes {:?}    Propective Nodes {:?}",
            time, working_nodes, prosp_nodes
        );
        //Do work and remove working nodes if they are done
        let mut indexes = vec![];
        for (idx, working_node) in working_nodes.iter().enumerate() {
            let worker = nodes.get_mut(working_node).unwrap();
            worker.1 -= 1;
            if worker.1 == 0 {
                indexes.push(idx);
            }
        }

        indexes.iter().for_each(|x| {
            working_nodes.swap_remove(*x);
        });

        // println!("Working Nodes after remove {:?}", working_nodes);

        // find next working nodes
        find_next_workers(&mut nodes, &working_nodes, &mut prosp_nodes);

        // println!("Propective Nodes after find {:?}", prosp_nodes);

        // Assign workers
        for prosp_node in &prosp_nodes {
            if working_nodes.capacity() > working_nodes.len() {
                working_nodes.push(*prosp_node);
            }
        }

        for working_node in &working_nodes {
            if let Some(pos) = prosp_nodes.iter().position(|x| x == working_node) {
                prosp_nodes.remove(pos);
            }
        }

        time += 1;

        if nodes.values().all(|x| x.1 == 0) {
            break;
        }
    }
    time
}

fn find_next_workers(
    nodes: &mut HashMap<char, (Vec<char>, u8)>,
    working_nodes: &Vec<char>,
    prosp_nodes: &mut Vec<char>,
) {
    nodes
        .iter()
        // Not being worked on
        .filter(|(y, _)| !working_nodes.contains(y))
        // Not already done
        .filter(|(_, (_, xx))| *xx > 0)
        // All decendants are done
        .filter(|(z, _)| {
            nodes
                .iter()
                .inspect(|cc| {
                    if **z == 'T' {
                        println!("Node {:?}", cc)
                    }
                })
                .filter(|(_, (aa, _))| aa.contains(&z))
                .inspect(|cc| {
                    if **z == 'T' {
                        println!("{:?} Requires {:?}", z, cc)
                    }
                })
                .all(|(_, (_, bb))| *bb == 0)
        })
        .for_each(|(next, _)| {
            if !prosp_nodes.contains(next) {
                println!("Found {:?} ", *next);
                prosp_nodes.push(*next);
            }
        });
    prosp_nodes.sort_unstable();
    prosp_nodes.dedup();
}

fn traverse(mut nodes: HashMap<char, (Vec<char>, bool)>, mut prosp_nodes: Vec<char>) -> String {
    let mut result = String::new();
    let mut curr_char = prosp_nodes[0];
    loop {
        result += curr_char.to_string().as_str();
        nodes.entry(curr_char).and_modify(|c| c.1 = true);
        let pos = prosp_nodes.iter().position(|x| x == &curr_char).unwrap();
        prosp_nodes.remove(pos);
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
