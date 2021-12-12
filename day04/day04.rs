use std::collections::VecDeque;
use std::env;
use std::fs;

type Board = [[i32; 5]; 5];

fn final_score(board: Board, picked: &Vec<i32>) -> i32 {
    let mut unmarked_sum = 0;
    for row in board {
        for val in row {
            if !picked.contains(&val) {
                unmarked_sum += val;
            }
        }
    }

    picked.last().unwrap() * unmarked_sum
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Supply input file.");
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    let mut lines = data.lines();

    // Read bingo numbers into a vec
    let mut numbers: VecDeque<i32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    //println!("{:#?}", numbers);

    // Parse bingo boards
    let mut boards: Vec<Board> = Vec::new();

    while lines.next().is_some() {
        let mut board = [[0i32; 5]; 5];

        // Read 5 rows
        for r in 0..5 {
            let mut row_chars = lines.next().unwrap().chars();

            // Read 5x 2 char numbers, skip space
            for c in 0..5 {
                let first = &row_chars.next().unwrap();
                let nc: String;
                if first == &' ' {
                    nc = row_chars.next().unwrap().to_string();
                } else {
                    nc = format!("{}{}", first, &row_chars.next().unwrap());
                }
                board[r][c] = nc.parse::<i32>().unwrap();
                row_chars.next();
            }
        }

        boards.push(board);
    }
    //println!("{:#?}", boards);

    // Start picking numbers. Must have used 5 though at least to even have a match.
    let mut picked: Vec<i32> = Vec::new();
    for _ in 0..5 {
        picked.push(numbers.pop_front().unwrap());
    }

    // Keep going until there is a winner
    // We then keep going after that for part 2.
    let mut first_winner: Option<Board> = None;
    let mut first_score: i32 = 0;
    let mut last_winner: Option<Board> = None;
    let mut last_score: i32 = 0;
    loop {
        // println!("\n\nPicked: {:?}", picked);
        // Check all boards, if no winner, then pick another number
        let mut found_boards: Vec<Board> = Vec::new();
        for board in boards.iter() {
            // Check rows
            let mut found: bool = true;
            for row in board {
                found = row
                    .iter()
                    .fold(true, |accum, val| accum && picked.contains(val));

                if found {
                    break;
                }
            }

            if found {
                let score: i32 = final_score(*board, &picked);
                // println!("Process winner board: {:?} ({})", *board, score);
                if first_winner.is_none() {
                    first_winner = Some(*board);
                    first_score = score;
                }
                last_winner = Some(*board);
                last_score = score;
                found_boards.push(*board);
                continue;
            }

            // Check columns
            for col_i in 0..5 {
                let col = board.map(|row| row[col_i]);

                found = col
                    .iter()
                    .fold(true, |accum, val| accum && picked.contains(val));

                if found {
                    break;
                }
            }

            if found {
                let score: i32 = final_score(*board, &picked);
                // println!("Process winner board: {:?} ({})", *board, score);
                if first_winner.is_none() {
                    first_winner = Some(*board);
                    first_score = score;
                }
                last_winner = Some(*board);
                last_score = score;
                found_boards.push(*board);
                continue;
            }
        }

        boards.retain(|&x| !found_boards.contains(&x));
        if boards.is_empty() {
            break;
        }

        // Pick another
        let next_num = numbers.pop_front();
        if next_num.is_none() {
            break;
        } else {
            picked.push(next_num.unwrap());
        }
    }

    if first_winner.is_none() {
        println!("Ran out of numbers, no winner!");
        std::process::exit(1);
    }

    println!("-- Part 1 --");
    println!("First winning board:");
    println!("{:#?}", first_winner.unwrap());

    // Calculate winning score
    // println!("Sum of unmarked: {}", unmarked_sum);
    println!("Final score: {}", first_score);

    println!("");
    println!("");
    println!("-- Part 2 --");

    println!("Last winning board:");
    println!("{:#?}", last_winner.unwrap());

    // Calculate winning score
    // println!("Sum of unmarked: {}", unmarked_sum);
    println!("Final score: {}", last_score);
}
