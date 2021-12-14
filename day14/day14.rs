use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("{} <input.txt> <times>", &args[0]);
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let times: u32 = args[2].parse().unwrap();

    let data = String::from_utf8_lossy(&file);
    let mut lines = data.lines();

    let p = lines.next().unwrap();
    let mut polymer_pairs = HashMap::new();
    let mut polymer_occ = HashMap::new();
    for c in p.chars() {
        *polymer_occ.entry(c).or_insert(0u64) += 1u64;
    }
    for pp in p.chars().collect::<Vec<char>>().windows(2) {
        *polymer_pairs
            .entry([pp[0], pp[1]])
            .or_insert(0u64) += 1u64;
    }

    let mut rules = HashMap::new();
    lines.next();
    for line in lines.by_ref() {
        let mut cc = line.chars();
        rules.insert(
            [cc.next().unwrap(), cc.next().unwrap()],
            cc.nth(4).unwrap(),
        );
    }

    for _ in 0..times {
        let mut new_pairs = polymer_pairs.clone();
        rules
            .iter()
            .for_each(|(from, to_char)| {
                if polymer_pairs.contains_key(from) {
                    let count = *polymer_pairs.get(from).unwrap();
                    let to1 = [from[0], *to_char];
                    let to2 = [*to_char, from[1]];
                    *new_pairs.get_mut(from).unwrap() -= count;
                    *new_pairs.entry(to1).or_insert(0u64) += count;
                    *new_pairs.entry(to2).or_insert(0u64) += count;
                    *polymer_occ.entry(*to_char).or_insert(0u64) += count;
                }
            });
        polymer_pairs = new_pairs;
    }

    let most_common = polymer_occ
        .iter()
        .max_by_key(|&(_, v)| v)
        .unwrap();
    println!("Most common: {:#?} @ {}x", most_common.0, most_common.1);

    let least_common = polymer_occ
        .iter()
        .min_by_key(|&(_, v)| v)
        .unwrap();
    println!("Least common: {:#?} @ {}x", least_common.0, least_common.1);

    println!("Score: {}", most_common.1 - least_common.1);
}
