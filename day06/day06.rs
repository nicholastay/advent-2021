use std::env;
use std::fs;
use std::collections::VecDeque;

const DEBUG_OUTPUT: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("{} <input.txt> <days>", &args[0]);
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    let sim_days: u32 = args[2]
        .parse()
        .expect("Could not parse number of days given.");

    // Approach: have a queue which we pop from, that keeps adding more children.
    // We keep the day number where the first child is born, and keep advancing from there
    // (0-indexed, so the number we read in works out).
    let mut queue: VecDeque<u32> = data
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<VecDeque<u32>>();
    if DEBUG_OUTPUT { println!("Initial state: {:?}", queue); }

    let mut count = queue.len() as u64;
    while !queue.is_empty() {
	if DEBUG_OUTPUT { println!("\nQueue state: {:?}", queue); }
	let mut day_idx = queue.pop_front().unwrap();
	if DEBUG_OUTPUT { println!("Processing day idx: {}", day_idx); }
	while day_idx < sim_days {
	    if DEBUG_OUTPUT { println!("-- Counting idx: {}", day_idx); }
	    count += 1;
	    let offset_idx = day_idx + 9; // Will birth total 9 later
	    if offset_idx < sim_days {
		queue.push_back(offset_idx);
		if DEBUG_OUTPUT { println!("-- Pushing for idx {}: day {}", day_idx, offset_idx); }
	    }
	    day_idx += 7;
	}
    }

    println!("Part 1 lanternfish count: {}", count);
}
