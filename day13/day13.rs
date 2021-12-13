use std::env;
use std::fs;

const WIDTH: usize = 1500;
const HEIGHT: usize = 1500;

type Map = [[bool; WIDTH]; HEIGHT];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{} <input.txt>", &args[0]);
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);
    let mut lines = data.lines();

    let mut map: Map = [[false; WIDTH]; HEIGHT];
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let mut it = line.split(',').map(|i| i.parse::<usize>().unwrap());
        let x = it.next().unwrap();
        let y = it.next().unwrap();

        map[y][x] = true;
    }

    for line in lines.by_ref() {
        let is_x: bool = line.chars().nth(11).unwrap() == 'x';
        let i: usize = line[13..].parse().unwrap();
        if is_x {
            fold_x(&mut map, i);
        } else {
            fold_y(&mut map, i);
        }

        // Part 1
        break;
    }

    println!(
        "{}",
        map
            .iter()
            .flatten()
            .filter(|&x| *x)
            .count()
    );
}

fn fold_x(map: &mut Map, x: usize) {
    for c in x+1..2*x+1 {
        let rp = reflect_point(c, x);
        assert!(rp < WIDTH);
        for r in 0..HEIGHT {
            let p = &mut map[r][c];
            if *p {
                *p = false;
                map[r][rp] = true;
            }
        }
    }
}

fn fold_y(map: &mut Map, y: usize) {
    for r in y+1..2*y+1 {
        let rp = reflect_point(r, y);
        assert!(rp < HEIGHT);
        for c in 0..WIDTH {
            let p = &mut map[r][c];
            if *p {
                *p = false;
                map[rp][c] = true;
            }
        }
    }
}

// v: x or y of the point
// rp: reflection point, with the respective x/y as used in v
fn reflect_point(v: usize, rp: usize) -> usize {
    let dir: i32 = if v > rp { -1 } else { 1 };
    let res: i32 = rp as i32 + dir * (rp as i32 - v as i32).abs();
    assert!(res >= 0);
    res as usize
}
