#![feature(nll)]

extern crate elapsed;
extern crate regex;

use elapsed::measure_time;
use regex::Regex;
use std::collections::VecDeque;

fn main() {
    let input = "452 players; last marble is worth 70784 points";

    assert_eq!(part1("9 players; last marble is worth 25 points"), 32);
    assert_eq!(part1("10 players; last marble is worth 1618 points"), 8317);
    assert_eq!(
        part1("13 players; last marble is worth 7999 points"),
        146_373
    );
    assert_eq!(part1("17 players; last marble is worth 1104 points"), 2764);
    assert_eq!(part1("21 players; last marble is worth 6111 points"), 54718);
    assert_eq!(part1("30 players; last marble is worth 5807 points"), 37305);
    println!("Part1 {:?}", part1(input));

    println!("Part2 {:?}", part2(input));
}

fn part2(input: &str) -> usize {
    let (elapsed, output) = measure_time(|| shared(input, 100));
    println!("Part2 game [{}] Elapsed = {}", input, elapsed);
    output
}

fn part1(input: &str) -> usize {
    let (elapsed, output) = measure_time(|| shared(input, 1));
    println!("Part1 game [{}] Elapsed = {}", input, elapsed);
    output
}

fn shared(input: &str, factor: usize) -> usize {
    let re = Regex::new(r"^(\d+)[^\d]+(\d+).*$").unwrap();

    if let Some(cap) = re.captures_iter(input).next() {
        let mut players = vec![0; cap[1].parse::<usize>().unwrap()];

        let last_marble = cap[2].parse::<usize>().unwrap() * factor;
        let mut board = VecDeque::with_capacity(last_marble);

        board.push_back(0);

        (1..=last_marble).for_each(|next_marble| {
            if next_marble % 23 != 0 {
                board.rotate_ccw(1);
                board.push_back(next_marble);
            } else {
                proc_23(&mut board, &mut players, next_marble)
            }
        });
        return players.into_iter().max().unwrap();
    }
    panic!("No capture!");
}

//Stolen for speed, Thanks
trait Circle<T> {
    fn rotate_cw(&mut self, i: usize);
    fn rotate_ccw(&mut self, i: usize);
}

impl<T> Circle<T> for VecDeque<T> {
    fn rotate_cw(&mut self, i: usize) {
        for _ in 0..i {
            if let Some(val) = self.pop_back() {
                self.push_front(val);
            }
        }
    }
    fn rotate_ccw(&mut self, i: usize) {
        for _ in 0..i {
            if let Some(val) = self.pop_front() {
                self.push_back(val);
            }
        }
    }
}

fn proc_23(board: &mut VecDeque<usize>, players: &mut Vec<usize>, new_marble: usize) {
    board.rotate_cw(7);
    let player = new_marble % players.len();
    players[player] += new_marble;
    if let Some(val) = board.pop_back() {
        players[player] += val;
    }
    board.rotate_ccw(1);
}
