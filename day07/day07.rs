use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{} <input.txt>", &args[0]);
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    let mut input: Vec<i32> = data
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    input.sort_unstable();

    // Approach: Use the median, as that would be in the middle of all
    // Choose one of them middle ones if even (as it doesn't matter, moved to each)
    let mid_index = input.len() / 2;
    let best_pos = input[mid_index];
    println!("Best position: {}", best_pos);

    println!(
        "Fuel consumed: {}",
        input
            .into_iter()
            .fold(0, |acc, x| acc + (best_pos - x).abs()),
    );
}
