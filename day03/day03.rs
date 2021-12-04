use std::env;
use std::fs;
use std::convert::TryInto;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Supply input file.");
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

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
    let mut gamma_rate: i32 = 0;
    let mut epsilon_rate: i32 = 0;
    for (i, ones) in occ.iter().enumerate() {
	let add_to: &mut i32;
	if ones > &(len - ones) {
	    // There are more ones than zeroes
	    // So add to the gamma rate
	    add_to = &mut gamma_rate;
	} else {
	    // There are more zeroes than ones
	    // So this adds to the epsilon rate
	    add_to = &mut epsilon_rate;
	}

	*add_to += i32::pow(2, (binary_len - i - 1).try_into().unwrap());
    }

    println!("Gamma: {}, Epsilon: {}", gamma_rate, epsilon_rate);
    println!("Multiplied: {}", gamma_rate * epsilon_rate);
}
