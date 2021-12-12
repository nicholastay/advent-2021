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

    let map = data
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let h = map.len();
    let w = map[0].len();

    let mut score: u32 = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            let mut adjacents: Vec<u32> = Vec::new();
            if i > 0 {
                adjacents.push(map[i - 1][j]);
            }
            if i < h - 1 {
                adjacents.push(map[i + 1][j]);
            }
            if j > 0 {
                adjacents.push(map[i][j - 1]);
            }
            if j < w - 1 {
                adjacents.push(map[i][j + 1]);
            }

            if adjacents.iter().all(|a| height < a) {
                score += 1 + height;
            }
        }
    }

    println!("Part 1 risk levels sum: {}", score);
}
