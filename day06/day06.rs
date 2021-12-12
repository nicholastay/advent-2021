use std::env;
use std::fs;

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

    let initial: Vec<usize> = data
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    // Approach: store count of fish in each timing.
    // After index 0 means birth.
    let mut timings: [u64; 9] = [0; 9];
    // Load initial into timings
    for i in initial {
        timings[i - 1] += 1;
    }

    // Note: first day has already been done by above.
    for _ in 1..sim_days {
        timings.rotate_left(1);
        timings[6] += timings[8]; // Add back in parents
    }

    println!("Lanternfish count: {}", timings.iter().sum::<u64>());
}
