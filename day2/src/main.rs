
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    let _test_string = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

    println!("{:?}", part1(input));
    println!("{}", part2(input));
}

fn part2(input: &str) -> String {
    let inputs = input.lines().collect::<Vec<_>>();
    let mut results = ("","");
    'outer: for val in &inputs {
        for other in &inputs {
            if compare(val, other) == 1 {
				results = (val,other);
				break 'outer
			};
        }
    }
    results.0.to_owned() + "\n" + results.1
}

fn compare(first: &str, other: &str) -> usize {
    let something = first
        .chars()
        .zip(other.chars())
        .map::<char, _>(|(x, y)| {
            if x == y {
                return ' '
            }
            'x'
        })
        .collect::<String>();

    something.trim().len()
}

fn part1(input: &str) -> u32 {
    let results = input.lines().fold((0, 0), |mut acc, x| {
        let mut counts = HashMap::new();
        x.chars().for_each(|y| {
            let count = counts.entry(y).or_insert(0);
            *count += 1;
        });
        let mut got_two = false;
        let mut got_three = false;
        for val in counts.values() {
            if *val == 2 && !got_two {
                got_two = true;
                acc.0 += 1;
            }
            if *val == 3 && !got_three {
                got_three = true;
                acc.1 += 1;
            }
        }
        acc
    });

    results.0 * results.1
}
