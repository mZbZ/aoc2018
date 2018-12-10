extern crate elapsed;

use elapsed::measure_time;

fn main() {
    let input = include_str!("../input");

    let _test_input = r"position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

    part1(_test_input, 5, 2);
    part1(input, 100_000, 10332);
}

// fn part2(input: &str) -> String {
//     let (elapsed, output) = measure_time(|| ));
//     println!("Part2 game [{}] Elapsed = {}", input, elapsed);
//     output
// }

fn part1(input: &str, steps: usize, draw: usize) {
    let (elapsed, output) = measure_time(|| {
        println!("Starting...");
        let mut vectors = input
            .lines()
            .map(|line| {
                let coords = line
                    .split(|c| c == '<' || c == '>' || c == ',')
                    .filter_map(|x| x.trim().parse::<isize>().ok())
                    .collect::<Vec<isize>>();
                (coords[1], coords[0], coords[3], coords[2])
            })
            .collect::<Vec<(isize, isize, isize, isize)>>();
        println!("Collecting bounds...");

        let mut min_area = (0, usize::max_value());
        for step in 0..=steps {
            for vector in &mut vectors {
                vector.0 += vector.2;
                vector.1 += vector.3;
            }

            let min_x = vectors.iter().min_by_key(|x| x.0).unwrap().0;
            let diff_x = (vectors.iter().max_by_key(|x| x.0).unwrap().0 - min_x) as usize;

            let min_y = vectors.iter().min_by_key(|x| x.1).unwrap().1;
            let diff_y = (vectors.iter().max_by_key(|y| y.1).unwrap().1 - min_y) as usize;
            if diff_x * diff_y < min_area.1 {
                // println!("Min area {:?}", min_area);
                min_area = (step, diff_x * diff_y);
            }
            if draw != 0 && step == draw {
                println!("Creating grid with dimensions ({},{})...", diff_x, diff_y);

                let mut grid = vec![vec![false; diff_y + 1]; diff_x + 1];
                println!("Created grid...");
                for (x, val_x) in grid.iter_mut().enumerate() {
                    for (y, val_y) in val_x.iter_mut().enumerate() {
                        // if y == 1 && x == 1 {
                        //     println!("Vec ({},{})", vectors[0].0, vectors[0].1);
                        //     println!("Grid ({},{})", x, y);
                        //     println!("Diff ({},{})", diff_x, diff_y);
                        // }

                        for vector in &vectors {
                            if vector.0 - min_x == x as isize && vector.1 - min_y == y as isize {
                                *val_y = true;
                                break;
                            } else {
                                *val_y = false;
                            }
                        }
                        if *val_y {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                    println!("");
                }
            }
        }
        min_area
    });
    println!("Part1 game [{:?}] Elapsed = {}", output, elapsed);
}
