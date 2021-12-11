use std::env;
use std::fs;

fn part1(data: &str) -> usize {
    // Part 1: only care about 1/4/7/8
    data
	.lines()
        .flat_map(|x| x.split(" | ").nth(1).unwrap().split(" "))
	.filter(|x| {
	    let l = x.len();
	    l == 7 || l == 4 || l == 2 || l == 3
	})
	.count()
}

fn part2(data: &str) -> usize {
    data
	.lines()
	.map(|x| {
	    let mut s = x.split(" | ");
	    let signals = s.next().unwrap().split(" ").collect::<Vec<&str>>();
	    let outputs = s.next().unwrap().split(" ").collect::<Vec<&str>>();

	    // This is a BIG mess. I don't know how to more cleanly manage this algorithm
	    // at the moment. Maybe I can think of a better way. But this does work to
	    // deduce where each number would be, and it does make sense to me.
	    
	    let fives = signals
		.iter()
		.filter(|x| x.len() == 5)
		.collect::<Vec<&&str>>();
	    let sixes = signals
		.iter()
		.filter(|x| x.len() == 6)
		.collect::<Vec<&&str>>();
	    // The number "2" must be these chars
	    let one = signals
		.iter()
		.find(|x| x.len() == 2)
		.unwrap();

	    // Out of the 5-display segment numbers, only '3' can have the two segments as in '2'
	    let three = fives
		.iter()
		.find(|x| one.chars().all(|c| x.contains(c)))
		.unwrap();

	    // Two must be the one which does not fully overlap any of the six-display segment ones
	    // i.e. '5' and '3' will overlap with 6 or 9 fully, but 3 won't
	    let two = fives
		.iter()
		.find(|x| {
		    !sixes
			.iter()
			.any(|s| x.chars().all(|c| s.contains(c)))
		})
		.unwrap();

	    // Five is then NOT three or two
	    let five = fives.iter().find(|&x| x != three && x != two).unwrap();

	    // Zero then does not overlap 5, but 6 and 9 do
	    let zero = sixes
		.iter()
		.find(|x| !five.chars().all(|c| x.contains(c)))
		.unwrap();

	    // Nine then overlaps 1
	    let nine = sixes
		.iter()
		.filter(|&x| x != zero)
		.find(|x| one.chars().all(|c| x.contains(c)))
		.unwrap();

	    let six = sixes
		.iter()
		.find(|&x| x != zero && x != nine)
		.unwrap();

	    // I'm too tired to try and find the right data structure.
	    // This should be good enough(TM) for now
	    let patterns = [
		zero,
		one,
		two,
		three,
		signals.iter().find(|x| x.len() == 4).unwrap(),
		five,
		six,
		signals.iter().find(|x| x.len() == 3).unwrap(),
		signals.iter().find(|x| x.len() == 7).unwrap(),
		nine,
	    ];

	    // Time to decode.
	    outputs
		.iter()
		.map(|&x| {
		    patterns
			.iter()
			.position(|&p| x.len() == p.len() && p.chars().all(|c| x.contains(c)))
			.unwrap()
		})
		.reduce(|acc, x| acc * 10 + x)
		.unwrap()
	})
	.sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{} <input.txt>", &args[0]);
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    // println!("{}", part1(&data));
    println!("{}", part2(&data));
}
