use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Supply input file.");
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    let mut it = data.lines();
    let mut prev: i32 = it.next().unwrap().parse::<i32>().unwrap();
    
    let mut increases: i32 = 0;
    for line in it {
        let new = line.parse::<i32>().unwrap();
        if new > prev {
            increases += 1;
        }

        prev = new;
    }

    println!("{}", increases);
}
