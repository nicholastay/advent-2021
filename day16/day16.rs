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

    let input = data.lines().next().unwrap();
    let mut bits = input
        .chars()
        .map(|c| c.to_digit(16).unwrap())
        .flat_map(hex_digit_to_bitarr);

    let mut version_total = 0;
    decode_packet(&mut bits, &mut version_total);

    println!("==============");
    println!("version total: {}", version_total);
}

fn hex_digit_to_bitarr(i: u32) -> [bool; 4] {
    [
        i >> 3 & 1 == 1,
        i >> 2 & 1 == 1,
        i >> 1 & 1 == 1,
        i      & 1 == 1,
    ]
}

fn read_bits(bits: &mut impl Iterator<Item=bool>, n: usize) -> u32 {
    bits.take(n)
        .fold(0, |acc, x| (acc << 1) + (if x { 1 } else { 0 }))
}

fn decode_packet(bits: &mut impl Iterator<Item=bool>, version_total: &mut u32) {
    println!("-- decode --");
    let version = read_bits(bits, 3);
    println!("version: {}", version);
    *version_total += version;

    let packet_type = read_bits(bits, 3);
    println!("type: {}", packet_type);
    if packet_type == 4 {
        // TODO: do something with this?
        let x = decode_literal(bits);
        println!("literal: {}", x);
    } else {
        let is_subpacket_type = bits.next().unwrap();
        if is_subpacket_type {
            // Length type ID = 1
            let count = read_bits(bits, 11) as usize;
            println!("subpacket type 1, count: {}", count);
            for _ in 0..count {
                decode_packet(bits, version_total);
            }
        } else {
            // Length type ID = 0
            let length = read_bits(bits, 15) as usize;
            println!("subpacket type 0, length: {}", length);
            // TODO: is there a better way to read out this 'x' amount of bits?
            let mut next_bits = bits.take(length)
                .collect::<Vec<bool>>()
                .into_iter()
                .peekable();
            while next_bits.peek().is_some() {
                decode_packet(&mut next_bits, version_total);
            }
        }
    }
}

fn decode_literal(bits: &mut impl Iterator<Item=bool>) -> u32 {
    let mut ans = 0;
    loop {
        let is_last = !bits.next().unwrap();
        ans <<= 4;
        ans += read_bits(bits, 4);
        if is_last {
            return ans;
        }
    }
}
