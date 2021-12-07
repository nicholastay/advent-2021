use std::env;
use std::fs;

const DEBUG_OUTPUT: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("{} <input.txt> <days>", &args[0]);
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    let sim_days: i32 = args[2]
        .parse()
        .expect("Could not parse number of days given.");

    let mut lanternfish: Vec<i32> = data
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    if DEBUG_OUTPUT { println!("Initial state: {:?}", lanternfish); }
    for day in 0..sim_days {
	let mut new_fish = 0;
	for fish in lanternfish.iter_mut() {
	    *fish -= 1;
	    if *fish < 0 {
		*fish = 6;
		new_fish += 1;
	    }
	}

	for _ in 0..new_fish {
	    lanternfish.push(8);
	}

	if DEBUG_OUTPUT { println!("After {:2} days: {:?}", day+1, lanternfish); }
    }

    if DEBUG_OUTPUT { println!(""); }
    println!("{} fish after {} days", lanternfish.len(), sim_days);
}
