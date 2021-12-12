use std::env;
use std::fs;

// Returns the offending character, or nothing.
fn check_line(line: &str) -> Option<char> {
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
                    return Some(c);
                }
            }
            _ => unreachable!(),
        }
    }

    None
}

fn char_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
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
            .filter_map(check_line)
            .map(char_score)
            .sum::<u32>()
    )
}
