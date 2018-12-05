#![feature(nll)]
#![feature(vec_remove_item)]

fn main() {
    let input = include_str!("../input");


    assert_eq!(part1("dabAcCaCBAcCcaDA"), 10);
	assert_eq!(part1("aA"), 0);
	assert_eq!(part1("abBA"), 0);
	assert_eq!(part1("abAB"), 4);
	assert_eq!(part1("aabAAB"), 6);
    println!("{:?}", part1(input));
	
    println!("{:?}", part2(input));
}

fn part2(input: &str) -> usize {
	let mut units = input.trim().chars().map(|x| x.to_ascii_lowercase()).collect::<Vec<char>>();
	units.sort_unstable();
	units.dedup();
	let mut min = usize::max_value();
	for unit in units {
		let input_removed = input.replace(unit,"").replace(unit.to_ascii_uppercase(),"");
		let this_len = part1(&input_removed);
		if this_len < min {
			min = this_len;
		}
	}
    min
}

fn part1(input: &str) -> usize {
    let mut char_vec = input.trim().chars().collect::<Vec<char>>();
    let mut idx = 1;
	
    while idx < char_vec.len() {
		
        if char_vec[idx].eq_ignore_ascii_case(&char_vec[idx - 1])
            && ((char_vec[idx].is_lowercase() && char_vec[idx - 1].is_uppercase())
                || (char_vec[idx - 1].is_lowercase() && char_vec[idx].is_uppercase()))
        {
			
            char_vec.remove(idx-1);
            char_vec.remove(idx-1);
			if idx > 1 {
				idx -= 1;
			}
        } else {
            idx += 1;
        }
    }
    char_vec.len()
}
