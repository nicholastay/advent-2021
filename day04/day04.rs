use std::env;
use std::fs;
use std::collections::VecDeque;

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
    let mut numbers: VecDeque<i32> = lines.next().unwrap().split(",").map(|i| i.parse::<i32>().unwrap()).collect();
    //println!("{:#?}", numbers);

    // Parse bingo boards
    type Board = [[i32; 5]; 5];
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
    let mut winner: Option<Board> = None;
    while !numbers.is_empty() {
	// Check all boards, if no winner, then pick another number
	for board in &boards {
	    // Check rows
	    let mut found: bool = true;
	    for row in board {
		found = row.iter().fold(true, |accum, val| {
		    accum && picked.contains(val)
		});

		if found {
		    break;
		}
	    }

	    if found {
		winner = Some(*board);
		break;
	    }

	    // Check columns
	    for col_i in 0..5 {
		let col = board.map(|row| row[col_i]);

		found = col.iter().fold(true, |accum, val| {
		    accum && picked.contains(val)
		});

		if found {
		    break;
		}
	    }

	    if found {
		winner = Some(*board);
		break;
	    }
	}

	if winner.is_some() {
	    break;
	} else {
	    // Pick another
	    picked.push(numbers.pop_front().unwrap());
	}
    }

    if winner.is_none() {
	println!("Ran out of numbers, no winner!");
	std::process::exit(1);
    }

    println!("Winning board:");
    println!("{:#?}", winner.unwrap());
    println!("");
    
    // Calculate winning score
    let just_picked = picked.last().unwrap();
    println!("Last number called: {}", just_picked);
    let mut unmarked_sum = 0;
    for row in winner.unwrap() {
	for val in row {
	    if !picked.contains(&val) {
		unmarked_sum += val;
	    }
	}
    }
    println!("Sum of unmarked: {}", unmarked_sum);

    println!("Final score: {}", just_picked * unmarked_sum);
}
