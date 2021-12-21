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
    let ans = decode_packet(&mut bits, &mut version_total);

    println!("version total: {}", version_total);
    println!("transmission: {}", ans);
}

fn hex_digit_to_bitarr(i: u32) -> [bool; 4] {
    [
        i >> 3 & 1 == 1,
        i >> 2 & 1 == 1,
        i >> 1 & 1 == 1,
        i      & 1 == 1,
    ]
}

fn read_bits(bits: &mut impl Iterator<Item=bool>, n: usize) -> u64 {
    bits.take(n)
        .fold(0, |acc, x| (acc << 1) + (if x { 1 } else { 0 }))
}

fn decode_packet(bits: &mut impl Iterator<Item=bool>, version_total: &mut u64) -> u64 {
    let version = read_bits(bits, 3);
    *version_total += version;

    let packet_type = read_bits(bits, 3);
    if packet_type == 4 {
        decode_literal(bits)
    } else {
        let accum_fn = match packet_type {
            0 => |acc: u64, x: u64| acc + x,
            1 => |acc: u64, x: u64| acc * x,
            2 => u64::min,
            3 => u64::max,
            5 => |acc: u64, x: u64| if acc > x { 1 } else { 0 },
            6 => |acc: u64, x: u64| if acc < x { 1 } else { 0 },
            7 => |acc: u64, x: u64| if acc == x { 1 } else { 0 },
            _ => unreachable!(),
        };

        let is_subpacket_type = bits.next().unwrap();
        if is_subpacket_type {
            // Length type ID = 1
            let count = read_bits(bits, 11) as usize;
            let mut ans = decode_packet(bits, version_total);
            for _ in 1..count {
                ans = accum_fn(ans, decode_packet(bits, version_total));
            }
            ans
        } else {
            // Length type ID = 0
            let length = read_bits(bits, 15) as usize;
            // TODO: is there a better way to read out this 'x' amount of bits?
            let mut next_bits = bits.take(length)
                .collect::<Vec<bool>>()
                .into_iter()
                .peekable();

            let mut ans = decode_packet(&mut next_bits, version_total);
            while next_bits.peek().is_some() {
                ans = accum_fn(ans, decode_packet(&mut next_bits, version_total));
            }
            ans
        }
    }
}

fn decode_literal(bits: &mut impl Iterator<Item=bool>) -> u64 {
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
