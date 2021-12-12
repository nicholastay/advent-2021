use std::env;
use std::fs;

enum LineResult {
    Corrupted(char),
    Incomplete(Vec<char>),
    Good,
}
use LineResult::*;

// Returns the offending character, or nothing.
fn check_line(line: &str) -> LineResult {
    let cs = line.chars();
    let mut closing_stack: Vec<char> = Vec::with_capacity(line.len());
    for c in cs {
        match c {
            '{' => closing_stack.push('}'),
            '(' => closing_stack.push(')'),
            '[' => closing_stack.push(']'),
            '<' => closing_stack.push('>'),
            '}' | ')' | ']' | '>' => {
                if c != closing_stack.pop().unwrap() {
                    return Corrupted(c);
                }
            }
            _ => unreachable!(),
        }
    }

    if closing_stack.is_empty() {
        Good
    } else {
        Incomplete(closing_stack)
    }
}

fn char_illegal_value(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn part1_check(line: &str) -> Option<char> {
    let res = check_line(line);
    // Ignore incomplete lines
    match res {
        Corrupted(c) => Some(c),
        _ => None,
    }
}

fn part2_check(line: &str) -> Option<Vec<char>> {
    let res = check_line(line);
    // Ignore corrupted lines
    match res {
        Incomplete(cs) => Some(cs),
        _ => None,
    }
}

fn char_completion_value(c: char) -> u32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

// Just pass in the raw completion stack.
// This will take care of the reversal.
fn chars_completion_score(cs: Vec<char>) -> u64 {
    cs
        .into_iter()
        .rev()
        .map(char_completion_value)
        .fold(0, |acc, x| acc * 5 + x as u64)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{} <input.txt>", &args[0]);
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    println!(
        "Part 1 total syntax error score: {}",
        data.lines()
            .filter_map(part1_check)
            .map(char_illegal_value)
            .sum::<u32>()
    );

    let mut part2_scores = data.lines()
        .filter_map(part2_check)
        .map(chars_completion_score)
        .collect::<Vec<u64>>();
    part2_scores.sort_unstable();
    println!("Part 2 middle completion score: {}", part2_scores[part2_scores.len() / 2]);
}
