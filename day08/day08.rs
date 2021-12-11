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

    // Part 1: only care about 1/4/7/8
    let count = data
	.lines()
        .flat_map(|x| x.split(" | ").nth(1).unwrap().split(" "))
	.filter(|x| {
	    let l = x.len();
	    l == 7 || l == 4 || l == 2 || l == 3
	})
	.count();

    println!("{}", count);
}
