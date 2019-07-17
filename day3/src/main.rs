extern crate regex;

use regex::Regex;

fn main() {
    let input = include_str!("../input");

    let _test_input = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    println!("{:?}", part1(input));
    println!("{}", part2(input));
}

fn part2(input: &str) -> usize {
    let mut fabric = vec![vec![0usize; 1000]; 1000];
    let re = Regex::new(r"^#(\d+).*@\s+(\d+),(\d+):\s(\d+)x(\d+)$").unwrap();

	let mut id = 0;
	
    input.lines().for_each(|z| {
        let coord = re.captures_iter(z).fold(vec![], |mut acc, zz| {
            zz.iter()
                .skip(2)
                .for_each(|zzz| acc.push(zzz.unwrap().as_str().parse::<usize>().unwrap()));
			
            acc
        });
        for x in 0..coord[2] {
            for y in 0..coord[3] {
                fabric[coord[0] + x][coord[1] + y] += 1usize;
            }
        }
    });
	
	input.lines().for_each(|z| {
		let mut has_overlap = false;
        let coord = re.captures_iter(z).fold(vec![], |mut acc, zz| {
            zz.iter()
                .skip(1)
                .for_each(|zzz| acc.push(zzz.unwrap().as_str().parse::<usize>().unwrap()));
			
            acc
        });
        for x in 0..coord[3] {
            for y in 0..coord[4] {
                 if has_overlap || fabric[coord[1] + x][coord[2] + y] > 1 {
					has_overlap = true;
				 }
            }
        }
		if !has_overlap {
			id  = coord[0];
		}
    });

    
    
    id
}

fn part1(input: &str) -> u32 {
    let mut fabric = vec![vec![0usize; 1000]; 1000];
    let re = Regex::new(r"^.*@\s+(\d+),(\d+):\s(\d+)x(\d+)$").unwrap();

    input.lines().for_each(|z| {
        let cap = re.captures_iter(z).fold(vec![], |mut acc, zz| {
            zz.iter()
                .skip(1)
                .for_each(|zzz| acc.push(zzz.unwrap().as_str().parse::<usize>().unwrap()));
            acc
        });
        for x in 0..cap[2] {
            for y in 0..cap[3] {
                fabric[cap[0] + x][cap[1] + y] += 1usize;
            }
        }
    });

    let mut count = 0;
    for row in &fabric {
        for cell in row {
			if *cell > 1 {
				count+= 1;
			}
		}
    }
    count
}
