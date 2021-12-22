use std::env;
use std::fs;
use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, Copy, Clone)]
struct NestVal {
    val: u32,
    depth: u32,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{} <input.txt>", &args[0]);
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    let pairs = data
        .lines()
        .map(parse_pair)
        .collect::<Vec<Vec<NestVal>>>();
    //println!("{:#?}", pairs.collect::<Vec<Vec<NestVal>>>());
    let n = pairs.len();

    // FIXME: I don't know how to make this elegant like just in part 1 with a reduce
    // Fighting the borrow checker isn't so much fun after a while...
    let mut pairs_sum = pairs[0].to_vec();
    for i in 1..n {
        pairs_sum = pairs_add(&pairs_sum, &pairs[i]);
    }
    println!("Part 1 full sum magnitude: {}", pairs_magnitude(&pairs_sum));

    let mut best_mag = u32::min_value();
    for i in 0..n {
        // Not commutative, so must test all the other way
        for j in 0..n {
            let mag = pairs_magnitude(&pairs_add(&pairs[i], &pairs[j]));
            if mag > best_mag {
                best_mag = mag;
            }
        }
    }
    println!("Part 2 best magnitude: {}", best_mag);
}

fn parse_pair(pair: &str) -> Vec<NestVal> {
    let mut v = Vec::new();
    parse_pair_aux(&mut pair.chars().peekable(), 0, &mut v);
    v
}

fn parse_expect(chars: &mut Peekable<Chars>, c: char) {
    if chars.next().unwrap() != c {
        unreachable!();
    }
}

fn parse_pair_aux(chars: &mut Peekable<Chars>, depth: u32, v: &mut Vec<NestVal>) {
    parse_expect(chars, '[');
    parse_pairval(chars, depth, v);
    parse_expect(chars, ',');
    parse_pairval(chars, depth, v);
    parse_expect(chars, ']');
}

fn parse_pairval(chars: &mut Peekable<Chars>, depth: u32, v: &mut Vec<NestVal>) {
    if *chars.peek().unwrap() == '[' {
        parse_pair_aux(chars, depth + 1, v);
    } else {
        v.push(
            NestVal {
                val: chars.next()
                          .unwrap()
                          .to_digit(10)
                          .unwrap(),
                depth,
            }
        );
    }
}

fn pairs_add(p1: &Vec<NestVal>, p2: &Vec<NestVal>) -> Vec<NestVal> {
    // Basic add the 2 first
    let mut v = p1
        .iter()
        .map(|nv| NestVal { val: nv.val, depth: nv.depth + 1 })
        .chain(
            p2
                .iter()
                .map(|nv| NestVal { val: nv.val, depth: nv.depth + 1 })
        )
        .collect::<Vec<NestVal>>();

    // Then perform explosions, splits, etc.
    let mut actioned = true;
    while actioned {
        actioned = false;

        // Always explode first if possible
        for i in 0..v.len() {
            if v[i].depth == 4 {
                // Explode: 'i' will be left of the pair
                // Assert that assumption + that the other pair exists
                assert!(v[i].depth == v[i+1].depth);
                if i > 0 {
                    v[i-1].val += v[i].val;
                }
                if i < v.len() - 2 {
                    v[i+2].val += v[i+1].val
                }
                v[i].val = 0;
                v[i].depth -= 1;
                v.remove(i+1);
                actioned = true;
                break;
            }
        }

        if actioned {
            continue;
        }

        // Then do splits
        for i in 0..v.len() {
            if v[i].val > 9 {
                let rem = v[i].val % 2;
                v[i].val /= 2;
                v[i].depth += 1;
                v.insert(i+1, NestVal { val: v[i].val + rem, depth: v[i].depth });
                actioned = true;
                break;
            }
        }
    }

    v
}

fn pairs_magnitude(v: &Vec<NestVal>) -> u32 {
    // There should be a more sound way to do this?
    let mut vv = v.clone();
    while vv.len() > 1 {
        for i in 0..vv.len()-1 {
            if vv[i].depth == vv[i+1].depth {
                vv[i].val *= 3;
                vv[i].val += 2 * vv[i+1].val;
                vv.remove(i + 1);
                if vv[i].depth == 0 {
                    assert!(vv.len() == 1);
                    return vv[0].val;
                } else {
                    vv[i].depth -= 1;
                }
                break;
            }
        }
    }
    unreachable!();
}
