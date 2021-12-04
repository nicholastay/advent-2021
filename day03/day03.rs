use std::env;
use std::fs;

fn part1(data: &str, gamma_rate: &mut i32, epsilon_rate: &mut i32) {
    *gamma_rate = 0;
    *epsilon_rate = 0;

    let binary_len = data.lines().next().unwrap().len();
    let mut occ: Vec<usize> = vec![0;binary_len];

    let mut len: usize = 0;
    for line in data.lines() {
	len += 1;
	let bits = line.chars();

	for (i, bit) in bits.enumerate() {
	    if bit == '1' {
		occ[i] += 1;
	    }
	}
    }

    //println!("{} / {:#?}", len, occ);

    // occ stores the number of 1's in that bit position
    // Store as decimal on the fly.
    for ones in occ.iter() {
	*gamma_rate <<= 1;
	*epsilon_rate <<= 1;
	
	let add_to: &mut i32;
	if ones > &(len - ones) {
	    // There are more ones than zeroes
	    // So add to the gamma rate
	    add_to = &mut *gamma_rate;
	} else {
	    // There are more zeroes than ones
	    // So this adds to the epsilon rate
	    add_to = &mut *epsilon_rate;
	}

	*add_to += 1;
    }
}

fn part2(data: &str, oxygen_gen: &mut i32, co2_scrub: &mut i32) {
    *oxygen_gen = 0;
    *co2_scrub = 0;

    let binary_len = data.lines().next().unwrap().len();

    // This kinda sucks, but whatever.
    let mut gamma_candidates = data.lines().collect::<Vec<_>>(); // Oxygen gen
    for i in 0..binary_len {
	//println!("doing pos {}", i);

	let gamma_bit: i32;
	let mut ones: usize = 0;
	for candidate in &gamma_candidates {
	    let cand_char = candidate.chars().nth(i).unwrap();
	    if cand_char == '1' {
		ones += 1
	    }
	}
	if ones >= &gamma_candidates.len() - ones {
	    gamma_bit = 1;
	} else {
	    gamma_bit = 0;

	}

	//println!("gamma bit {}", gamma_bit);

	let mut new_gamma_candidates: Vec<&str> = Vec::new();
	for candidate in &gamma_candidates {
	    let cand_char = candidate.chars().nth(i).unwrap();
	    let cand_bit = cand_char.to_digit(10).unwrap() as i32;
	    //println!("cand bit {}", cand_bit);

	    if cand_bit != gamma_bit {
		//println!("gamma removing {}", candidate);
	    } else {
		new_gamma_candidates.push(candidate);
		//println!("gamma keeping {}", candidate);
	    }
	}

	gamma_candidates = new_gamma_candidates;
	//println!("new gamma cands: {:#?}", gamma_candidates);

	if gamma_candidates.len() == 1 {
	    break;
	}
	
	//println!("------");
    }

    //println!("----------------------------------------");

    let mut epsilon_candidates = data.lines().collect::<Vec<_>>(); // CO2 scrub
    for i in 0..binary_len {
	//println!("doing pos {}", i);

	let epsilon_bit: i32;
	let mut ones: usize = 0;
	for candidate in &epsilon_candidates {
	    let cand_char = candidate.chars().nth(i).unwrap();
	    if cand_char == '1' {
		ones += 1
	    }
	}
	if ones >= &epsilon_candidates.len() - ones {
	    epsilon_bit = 0;
	} else {
	    epsilon_bit = 1;

	}

	//println!("epsilon bit {}", epsilon_bit);

	let mut new_epsilon_candidates: Vec<&str> = Vec::new();
	for candidate in &epsilon_candidates {
	    let cand_char = candidate.chars().nth(i).unwrap();
	    let cand_bit = cand_char.to_digit(10).unwrap() as i32;
	    //println!("cand bit {}", cand_bit);

	    if cand_bit != epsilon_bit {
		//println!("epsilon removing {}", candidate);
	    } else {
		new_epsilon_candidates.push(candidate);
		//println!("epsilon keeping {}", candidate);
	    }
	}

	epsilon_candidates = new_epsilon_candidates;
	
	//println!("new epsilon cands: {:#?}", epsilon_candidates);

	if epsilon_candidates.len() == 1 {
	    break;
	}

	//println!("------");
    }

    *oxygen_gen = i32::from_str_radix(gamma_candidates[0], 2).unwrap();
    *co2_scrub = i32::from_str_radix(epsilon_candidates[0], 2).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Supply input file.");
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    let mut gamma_rate: i32 = 0;
    let mut epsilon_rate: i32 = 0;
    part1(&data, &mut gamma_rate, &mut epsilon_rate);
    println!("Gamma: {}, Epsilon: {}", gamma_rate, epsilon_rate);
    println!("Multiplied: {}", gamma_rate * epsilon_rate);

    let mut oxygen_gen: i32 = 0;
    let mut co2_scrub: i32 = 0;
    part2(&data, &mut oxygen_gen, &mut co2_scrub);
    println!("Oxygen gen: {}, CO2 scrub: {}", oxygen_gen, co2_scrub);
    println!("Multiplied: {}", oxygen_gen * co2_scrub);
}
