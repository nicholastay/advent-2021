use std::env;
use std::fs;

fn part1(data: &str) -> i32 {
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

    increases
}

// Note: there was no real reason to do it like this for this challenge
// But I wanted an excuse to mess with more features in Rust as I learn
// it, so yeah.
struct SlidingWindow {
    vec: Vec<i32>,
    idx: usize,
}

impl SlidingWindow {
    fn insert(&mut self, val: i32) {
        self.vec[self.idx] = val;
        self.idx += 1;

        if self.idx >= 3 {
            self.idx = 0;
        }
    }

    fn sum(&self) -> i32 {
        self.vec.iter().sum()
    }
}

fn part2(data: &str) -> i32 {
    let mut it = data.lines();
    let mut sw = SlidingWindow {
        vec: vec![0; 3],
        idx: 0,
    };

    for _ in 0..3 {
        sw.insert(it.next().unwrap().parse::<i32>().unwrap());
    }
    let mut prev_sum: i32 = sw.sum();

    let mut increases: i32 = 0;
    for line in it {
        let new = line.parse::<i32>().unwrap();
        sw.insert(new);

        let s = sw.sum();
        if s > prev_sum {
            increases += 1;
        }

        prev_sum = s;
    }

    increases
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Supply input file.");
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    println!("Part 1 increases: {}", part1(&data));
    println!("Part 2 increases: {}", part2(&data));
}
