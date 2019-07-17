fn main() {
    let input = include_str!("../input");

    let _test_string = "+1
-2
+3
+1";

    println!("Test Part1: {:?}", part1(_test_string));
    println!("Part1: {:?}", part1(input));
    println!("Test Part2: {:?}", part2(_test_string));
    println!("Part2: {:?}", part2(input));
}

fn part2(input: &str) -> i32 {
    let mut results: Vec<i32> = vec![0i32];
    loop {
        let _something: Option<i32> =
            input
                .lines()
                .try_fold(*results.last().unwrap(), |mut acc: i32, x| {
                    acc += x.parse::<i32>().unwrap();
                    if results.contains(&acc) {
                        results.push(acc);
                        return None;
                    }
                    results.push(acc);
                    Some(acc)
                });
        if _something.is_none() {
            break;
        }
    }
    *results.last().unwrap()
}

fn part1(input: &str) -> i32 {
    input.lines().fold(0i32, |mut acc, x| {
        acc += x.parse::<i32>().unwrap();
        acc
    })
}
