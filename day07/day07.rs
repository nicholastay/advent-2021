use std::cmp;
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

    println!("Part 1 fuel consumed: {}", part1(&input),);
    println!("");
    println!("Part 2 fuel consumed: {}", part2(&input),);
}

fn part1(input: &Vec<i32>) -> i64 {
    // Approach: Use the median, as that would be in the middle of all
    // Choose one of them middle ones if even (as it doesn't matter, moved to each)
    let mid_index = input.len() / 2;
    let best_pos = input[mid_index] as i64;
    println!("Best part 1 position: {}", best_pos);

    input
        .iter()
        .fold(0, |acc, &x| acc + (best_pos - x as i64).abs())
}

fn part2(input: &Vec<i32>) -> i64 {
    // This kinda sucks?
    let max_pos = *input.iter().max().unwrap();
    (0..max_pos).fold(i64::MAX, |acc, pos| {
        cmp::min(
            acc,
            input
                .iter()
                .fold(0, |acc, &x| acc + triangle_n((pos as i64 - x as i64).abs())),
        )
    })
}

// The new part 2 fuel consumption function is just triangular numbers.
// i.e. if 11 distance, it is 11 + 10 + 9 ... + 1.
fn triangle_n(n: i64) -> i64 {
    n * (n + 1) / 2
}
