use std::env;
use std::fs;

fn part1(data: &str, horiz: &mut i32, depth: &mut i32) {
    for line in data.lines() {
        let mut tokens = line.split(" ");

        let direction = tokens.next().unwrap();
        let val = tokens.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => *horiz += val,
            "down" => *depth += val,
            "up" => *depth -= val,
            _ => {
                println!("unreachable, panic");
                std::process::exit(1);
            }
        }
    }
}

fn part2(data: &str, horiz: &mut i32, depth: &mut i32) {
    let mut aim: i32 = 0;

    for line in data.lines() {
        let mut tokens = line.split(" ");

        let direction = tokens.next().unwrap();
        let val = tokens.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => {
                *horiz += val;
                *depth += aim * val;
            }
            "down" => aim += val,
            "up" => aim -= val,
            _ => {
                println!("unreachable, panic");
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Supply input file.");
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    let mut horiz: i32 = 0;
    let mut depth: i32 = 0;

    part1(&data, &mut horiz, &mut depth);
    println!("Part 1");
    println!("Horizontal position: {}, Depth: {}", horiz, depth);
    println!("Multiplied: {}", horiz * depth);
    println!("");

    horiz = 0;
    depth = 0;
    part2(&data, &mut horiz, &mut depth);
    println!("Part 2");
    println!("Horizontal position: {}, Depth: {}", horiz, depth);
    println!("Multiplied: {}", horiz * depth);
}
