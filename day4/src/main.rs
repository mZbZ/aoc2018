
extern crate chrono;
extern crate regex;

use std::cmp::Ordering;
use std::collections::HashMap;

use chrono::prelude::*;
use regex::Captures;
use regex::Regex;

fn main() {
    let input = include_str!("../input");

    let _test_input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"^\[(\d+)-(\d+)-(\d+)\s(\d+):(\d+)\]\s(G|w|f).+?(\d+|$).*$").unwrap();
    let mut records: Vec<TimeRecord> = vec![];

    //use std::time::Instant;
    //let now = Instant::now();

    input
        .lines()
        .for_each(|z| records.push(TimeRecord::new(re.captures(z).unwrap())));
    records.sort();

    let mut id_records = HashMap::new();
    let mut id_key = 0;
    let mut asleep_start = 0;

    for rec in records {
        match rec.state {
            GuardState::Starting(x) => id_key = x,
            GuardState::Asleep => asleep_start = rec.date.minute(),
            GuardState::Awake => {
                let id_rec = id_records.entry(id_key).or_insert(HashMap::new());
                for asleep_mins in asleep_start..rec.date.minute() {
                    let min_rec = id_rec.entry(asleep_mins).or_insert(0);
                    *min_rec += 1;
                }
            }
        }
    }
	
	
	let mut cur_max = 0;
	let mut min_max = 0;
	let mut id_max = 0;
	for (id,mins) in id_records {
		let max = mins.into_iter().max_by_key(|x| x.1).unwrap().clone();
		if max.1 > cur_max {
			id_max = id;
			min_max = max.0;
			cur_max = max.1;
		}
		
	}
    

    //let elapsed = now.elapsed();
    //let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    //println!("Seconds: {}", sec);

    id_max * min_max
}

#[derive(Debug, PartialEq, Eq)]
enum GuardState {
    Starting(u32),
    Asleep,
    Awake,
}

#[derive(Debug, PartialEq, Eq)]
struct TimeRecord {
    date: DateTime<Utc>,
    state: GuardState,
}

impl TimeRecord {
    fn new(args: Captures) -> TimeRecord {
        TimeRecord {
            date: Utc
                .ymd(
                    args.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    args.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                    args.get(3).unwrap().as_str().parse::<u32>().unwrap(),
                )
                .and_hms(
                    args.get(4).unwrap().as_str().parse::<u32>().unwrap(),
                    args.get(5).unwrap().as_str().parse::<u32>().unwrap(),
                    0,
                ),
            state: match args.get(6).unwrap().as_str() {
                "G" => GuardState::Starting(args.get(7).unwrap().as_str().parse::<u32>().unwrap()),
                "w" => GuardState::Awake,
                "f" => GuardState::Asleep,
                _ => panic!("Should never happen"),
            },
        }
    }
}

impl Ord for TimeRecord {
    fn cmp(&self, other: &TimeRecord) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for TimeRecord {
    fn partial_cmp(&self, other: &TimeRecord) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"^\[(\d+)-(\d+)-(\d+)\s(\d+):(\d+)\]\s(G|w|f).+?(\d+|$).*$").unwrap();
    let mut records: Vec<TimeRecord> = vec![];

    //use std::time::Instant;
    //let now = Instant::now();

    input
        .lines()
        .for_each(|z| records.push(TimeRecord::new(re.captures(z).unwrap())));
    records.sort();

    let mut id_records = HashMap::new();
    let mut id_key = 0;
    let mut asleep_start = 0;

    for rec in records {
        match rec.state {
            GuardState::Starting(x) => id_key = x,
            GuardState::Asleep => asleep_start = rec.date.minute(),
            GuardState::Awake => {
                let id_rec = id_records.entry(id_key).or_insert(HashMap::new());
                for asleep_mins in asleep_start..rec.date.minute() {
                    let min_rec = id_rec.entry(asleep_mins).or_insert(0);
                    *min_rec += 1;
                }
            }
        }
    }
	
	
	let mut cur_total = 0;
	let mut min_max = (0u32,0);
	let mut id_max = 0;
	for (id,mins) in id_records {
		let mut total = 0; 
		for (_,count) in &mins {
			total += count;
		}
		if total > cur_total {
			min_max = mins.into_iter().max_by_key(|x| x.1).unwrap().clone();
			id_max = id;
			cur_total = total;
		}
		
	}
    

    //let elapsed = now.elapsed();
    //let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    //println!("Seconds: {}", sec);

    id_max * min_max.0
}
