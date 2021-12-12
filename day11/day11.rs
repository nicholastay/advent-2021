use std::env;
use std::fs;

const STEPS: usize = 100;
const HEIGHT: usize = 10;
const WIDTH: usize = 10;

type Map = Vec<Vec<MapValue>>;
type Point = (usize, usize);

struct MapValue {
    value: u32,
    flashed: bool,
}

fn adjacent_points((r, c): Point) -> Vec<Point> {
    let mut adjacents: Vec<Point> = Vec::new();
    if r > 0 {
        adjacents.push((r - 1, c));
        if c > 0 {
            adjacents.push((r - 1, c - 1));
        }
    }
    if r < HEIGHT - 1 {
        adjacents.push((r + 1, c));
        if c < WIDTH - 1 {
            adjacents.push((r + 1, c + 1));
        }
    }
    if c > 0 {
        adjacents.push((r, c - 1));
        if r < HEIGHT - 1 {
            adjacents.push((r + 1, c - 1))
        }
    }
    if c < WIDTH - 1 {
        adjacents.push((r, c + 1));
        if r > 0 {
            adjacents.push((r - 1, c + 1))
        }
    }

    adjacents
}

fn flash_point(map: &mut Map, (r, c): Point, count: &mut u32) {
    let p = &mut map[r][c];
    if !p.flashed {
        p.value += 1;

        if p.value > 9 {
            *count += 1;
            p.value = 0;
            p.flashed = true;

            adjacent_points((r, c))
                .iter()
                .for_each(|&(r2, c2)| flash_point(map, (r2, c2), count));
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{} <input.txt>", &args[0]);
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    let mut map = data
        .lines()
        .map(|x| {
            x.chars()
             .map(|c| MapValue { value: c.to_digit(10).unwrap(), flashed: false })
             .collect::<Vec<MapValue>>()
        })
        .collect::<Map>();

    let mut flash_count: u32 = 0;
    for _ in 0..STEPS {
        for r in 0..HEIGHT {
            for c in 0..WIDTH {
                map[r][c].value += 1;
            }
        }

        for r in 0..HEIGHT {
            for c in 0..WIDTH {
                if map[r][c].value > 9 {
                    flash_point(&mut map, (r, c), &mut flash_count);
                }
            }
        }

        for r in 0..HEIGHT {
            for c in 0..WIDTH {
                map[r][c].flashed = false;
            }
        }
    }

    println!("Part 1 flash count: {}", flash_count);
}
