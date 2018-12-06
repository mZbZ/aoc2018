#![feature(nll)]

use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    assert_eq!(
        part1(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"
        ),
        17
    );

    println!("{:?}", part1(input));

    assert_eq!(
        part2(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9",
            32
        ),
        16
    );
    println!("{:?}", part2(input, 10_000));
}

fn part2(input: &str, param: usize) -> usize {
    let coords = input
        .lines()
        .map(|x| {
            (
                x.split(", ")
                    .map(|y| y.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
                0,
            )
        })
        .collect::<HashMap<Vec<_>, usize>>();

    let b_points = get_boundry_points(&coords);

    let mut grid: HashMap<(usize, usize), usize> = HashMap::new();
    for n in b_points.0..=b_points.2 {
        for m in b_points.1..=b_points.3 {
            count_coord_distance(&mut grid, &coords, n, m);
        }
    }

    // println!("{:?}", grid);
    grid.values().fold(0, |mut acc, &x| {
        if x < param {
            acc += 1
        };
        acc
    })
}

fn part1(input: &str) -> usize {
    let mut coords = input
        .lines()
        .map(|x| {
            (
                x.split(", ")
                    .map(|y| y.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
                0,
            )
        })
        .collect::<HashMap<Vec<_>, usize>>();

    let b_points = get_boundry_points(&coords);
    eliminate_infinite(&mut coords, &b_points);

    for n in b_points.0..=b_points.2 {
        for m in b_points.1..=b_points.3 {
            find_and_count_closest(&mut coords, n, m);
        }
    }
    coords
        .into_iter()
        .filter(|x| x.1 != usize::max_value())
        .max_by_key(|x| x.1)
        .unwrap()
        .1
}

fn get_boundry_points(coords: &HashMap<Vec<usize>, usize>) -> (usize, usize, usize, usize) {
    (
        coords.iter().min_by_key(|a| a.0[0]).unwrap().0[0],
        coords.iter().min_by_key(|a| a.0[1]).unwrap().0[1],
        coords.iter().max_by_key(|a| a.0[0]).unwrap().0[0],
        coords.iter().max_by_key(|a| a.0[1]).unwrap().0[1],
    )
}

fn eliminate_infinite(
    coords: &mut HashMap<Vec<usize>, usize>,
    mins_maxs: &(usize, usize, usize, usize),
) {
    coords
        .iter_mut()
        .filter(|a| {
            (a.0[0] == mins_maxs.0 || a.0[1] == mins_maxs.1)
                || (a.0[0] == mins_maxs.2 || a.0[1] == mins_maxs.3)
        })
        .for_each(|b| *b.1 = usize::max_value());
}

fn find_and_count_closest<'a>(
    coords: &'a mut HashMap<Vec<usize>, usize>,
    x_coord: usize,
    y_coord: usize,
) {
    let min_point = coords
        .iter()
        .fold((isize::max_value(), None, true), |mut acc, x| {
            let distance = (x.0[0] as isize - x_coord as isize).abs()
                + (x.0[1] as isize - y_coord as isize).abs();
            if distance < acc.0 {
                acc.0 = distance;
                acc.1 = Some(x.0.clone());
                acc.2 = true;
            } else if distance == acc.0 {
                acc.2 = false;
            }
            acc
        });
    if min_point.2 {
        if let Some(count) = coords.get_mut(&min_point.1.unwrap()) {
            *count = count.saturating_add(1);
        }
    }
}

fn count_coord_distance<'a>(
    grid: &'a mut HashMap<(usize, usize), usize>,
    coords: &'a HashMap<Vec<usize>, usize>,
    x_coord: usize,
    y_coord: usize,
) {
    coords.iter().for_each(|x| {
        let count = grid.entry((x_coord, y_coord)).or_insert(0);
        *count += (x.0[0] as isize - x_coord as isize).abs() as usize
            + (x.0[1] as isize - y_coord as isize).abs() as usize;
    });
}
