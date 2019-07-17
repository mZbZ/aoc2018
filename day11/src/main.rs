extern crate elapsed;
extern crate rayon;

use elapsed::measure_time;
use rayon::prelude::*;

fn main() {
    let input = 9306;

    assert_eq!(calc_power_level(3, 5, 8), 4);

    assert_eq!(part1(42), ((21, 61), 3, 30));
    assert_eq!(part1(18), ((33, 45), 3, 29));
    println!("Part1 {:?}", part1(input));

    assert_eq!(part2(18), ((90, 269), 16, 113));
    assert_eq!(part2(42), ((232, 251), 12, 119));

    println!("Part2 {:?}", part2(input));
}

fn part2(input: usize) -> ((usize, usize), usize, isize) {
    let (elapsed, output) = measure_time(|| {
        (1..300usize)
            .into_par_iter()
            .map(|ss| shared(input, ss))
            .max_by_key(|&x| x.2)
            .unwrap()
    });
    println!("Part2 Time Elapsed = {}", elapsed);
    output
}

fn part1(input: usize) -> ((usize, usize), usize, isize) {
    let (elapsed, output) = measure_time(|| shared(input, 3));
    println!("Part1 Time Elapsed = {}", elapsed);
    output
}

fn shared(input: usize, sample_size: usize) -> ((usize, usize), usize, isize) {
    let grid = (1..=300)
        .map(|x| {
            (1..=300)
                .map(|y| calc_power_level(x, y, input))
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();

    let mut max = ((0, 0), sample_size, 0);
    for x in 0..(grid.len() - sample_size) {
        for y in 0..(grid.len() - sample_size) {
            let total: isize = grid
                .iter()
                .skip(x)
                .take(sample_size)
                .fold(0isize, |sum, xx| {
                    sum + xx.iter().skip(y).take(sample_size).sum::<isize>()
                });
            if total > max.2 {
                max = ((x + 1, y + 1), sample_size, total)
            }
        }
    }
    max
}

fn calc_power_level(x: usize, y: usize, serial: usize) -> isize {
    let mut result = (((x + 10) * y) + serial) * (x + 10);
    result -= result % 100;
    ((result / 100) % 10) as isize - 5
}
