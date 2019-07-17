extern crate elapsed;
extern crate regex;

use elapsed::measure_time;

fn main() {
    let input = include_str!("../input");

    assert_eq!(part1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 138);
    println!("Part1 {:?}", part1(input));

    assert_eq!(part2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 66);

    println!("Part2 {:?}", part2(input));
}

fn part2(input: &str) -> usize {
    let (elapsed, output) = measure_time(|| {
        let input_v: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        get_node_part2(&input_v, &mut 0, &mut None)
    });
    println!("Part2 Time Elapsed = {}", elapsed);
    output
}

fn part1(input: &str) -> usize {
    let (elapsed, output) = measure_time(|| {
        let input_v: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        get_node(&input_v, &mut 0, &mut 0)
    });
    println!("Part1 Time Elapsed = {}", elapsed);
    output
}

fn get_node(input: &[usize], idx: &mut usize, metadata: &mut usize) -> usize {
    // println!("Found group at {}", *idx);
    let mut c_nodes = input[*idx];
    *idx += 1;
    let mut m_nodes = input[*idx];
    *idx += 1;
    while c_nodes > 0 {
        get_node(input, idx, metadata);
        c_nodes -= 1;
    }
    while m_nodes > 0 {
        *metadata += input[*idx];
        *idx += 1;
        m_nodes -= 1;
    }
    *metadata
}

fn get_node_part2(input: &[usize], idx: &mut usize, child_nodes: &mut Option<Vec<usize>>) -> usize {
    let mut c_nodes = input[*idx];
    *idx += 1;
    let mut m_nodes = input[*idx];
    *idx += 1;
    let mut metadata = 0;
    if c_nodes == 0 {
        while m_nodes > 0 {
            metadata += input[*idx];
            *idx += 1;
            m_nodes -= 1;
        }
        match child_nodes {
            Some(child_nodes_vec) => child_nodes_vec.push(metadata),
            None => panic!("Should never happen"),
        }
    } else {
        let mut new_child_nodes = Some(vec![]);

        while c_nodes > 0 {
            get_node_part2(input, idx, &mut new_child_nodes);
            c_nodes -= 1;
        }

        let mut child_idx = vec![];
        while m_nodes > 0 {
            child_idx.push(input[*idx] - 1);
            *idx += 1;
            m_nodes -= 1;
        }

        for idx in child_idx {
            metadata += new_child_nodes.as_ref().unwrap().get(idx).unwrap_or(&0);
        }

        child_nodes.as_mut().unwrap_or(&mut vec![]).push(metadata);
    }

    metadata
}
