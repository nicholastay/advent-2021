use std::env;
use std::fs;

const MAX_INDEX: usize = 244;

// Adjancency matrix like structure
type Map = [[bool; MAX_INDEX + 1]; MAX_INDEX + 1];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{} <input.txt>", &args[0]);
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    // Generate the nodes and map
    // Let 0 = disconnected, 1 = start node, 2 = end node, i = linked to node #i
    let mut map: Map = [[false; MAX_INDEX + 1]; MAX_INDEX + 1];
    data
        .lines()
        .for_each(|line| {
            let mut s = line.split("-");

            let from = string_val(s.next().unwrap());
            assert!(from <= MAX_INDEX);
            let to = string_val(s.next().unwrap());
            assert!(to <= MAX_INDEX);

            map[from][to] = true;
            map[to][from] = true;
        });

    // Go through the paths and see if it visits all small caves
    println!("Part 1 path count: {}", scan_part1_paths(&map, 1, &Vec::new()));
}

fn string_val(s: &str) -> usize {
    if s == "start" {
        1
    } else if s == "end" {
        2
    } else {
        // Use sums for keeping track of multiletters
        s
            .chars()
            .map(|x| x as usize)
            .sum()
    }
}

#[allow(dead_code)]
fn val_is_small(x: usize) -> bool {
    // Count start, end as small so can't visit that again
    x == 1 || x == 2 || (x >= 97 && x <= 122) || x >= 194
}

#[allow(dead_code)]
fn val_is_big(x: usize) -> bool {
    (x >= 65 && x <= 90) || (x >= 130 && x <= 180)
}

fn scan_part1_paths(map: &Map, curr: usize, visited: &Vec<usize>) -> usize {
    if curr == 2 {
        if visited.into_iter().any(|x| val_is_small(*x)) {
            // println!("found {} -- visited: {:#?}", curr, visited);
            return 1;
        } else {
            return 0;
        }
    }

    // println!("scanning {} -- visited: {:#?}", curr, visited);

    // Can visit big caves as much as we want, so don't count them here
    let mut new_visited: Vec<usize> = visited.clone();
    if val_is_small(curr) {
        new_visited.push(curr);
    }

    map[curr]
        .iter()
        .enumerate()
        .filter(|&(_, conn)| *conn)
        .map(|(i, _)| i)
        .filter(|i| !visited.contains(i))
        .map(|i| scan_part1_paths(map, i, &new_visited))
        .sum::<usize>()
}
