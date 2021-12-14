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

    let mut polymer: Vec<char> = lines.next().unwrap().chars().collect();
    let mut occ = HashMap::new();
    for &c in &polymer {
        *occ.entry(c).or_insert(1) += 1;
    }

    let mut rules = HashMap::new();
    lines.next();
    for line in lines.by_ref() {
        rules.insert(
            line[..2].to_string(),
            line.chars().nth(6).unwrap(),
        );
    }

    // println!("{:#?}", polymer);
    // println!("{:#?}", rules);
    // polymer.windows(2).for_each(|x| println!("{:#?}", x));

    for _ in 0..times {
        let mut first = true; // This is so weird but I can't be stuffed thinking of a better way.
        polymer = polymer
            .windows(2)
            .flat_map(|pair| {
                let s = pair.iter().collect::<String>();
                match rules.get(&s) {
                    Some(c) => {
                        *occ.entry(*c).or_insert(1) += 1;
                        if first {
                            first = false;
                            Vec::from([pair[0], *c, pair[1]])
                        } else {
                            Vec::from([*c, pair[1]])
                        }
                    },
                    None => { first = false; Vec::from(pair) },
                }
            })
            .collect();
    }

    println!("Polymer: {}", polymer.into_iter().collect::<String>());
    // println!("Occurrences: {:#?}", occ);

    let most_common = occ
        .iter()
        .max_by_key(|&(_, v)| v)
        .unwrap();
    println!("Most common: {} @ {}x", most_common.0, most_common.1);

    let least_common = occ
        .iter()
        .min_by_key(|&(_, v)| v)
        .unwrap();
    println!("Least common: {} @ {}x", least_common.0, least_common.1);

    println!("Score: {}", most_common.1 - least_common.1);
}
