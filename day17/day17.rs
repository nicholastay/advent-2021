use std::env;
use std::fs;

// Brute Force constants
// FIXME: There HAS to be a better way to do this problem (with brain and maths)
const BF_DX_MIN: i32 = 1;
const BF_DX_MAX: i32 = 500;
const BF_DY_MIN: i32 = -250; // >0 for part1 since trick shot. <0 possible for enumerating.
const BF_DY_MAX: i32 = 250;
const BF_STEPS_MAX: i32 = 200;

struct Rect {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{} <input.txt>", &args[0]);
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    // TODO: Is there a more elegant way of parsing this stuff?...
    let target_parse = data
        .lines()
        .next()
        .unwrap()[13..]
        .split(", ")
        .flat_map(|x| {
            let mut s = x[2..].split("..");
            [s.next().unwrap().parse::<i32>().unwrap(), s.next().unwrap().parse::<i32>().unwrap()]
        })
        .collect::<Vec<i32>>();
    let area = Rect { x1: target_parse[0], x2: target_parse[1], y1: target_parse[2], y2: target_parse[3] };

    let mut best_vel = None;
    let mut best_y = i32::min_value();
    let mut vels_count = 0;
    for dx in BF_DX_MIN..BF_DX_MAX+1 {
        for dy in BF_DY_MIN..BF_DY_MAX+1 {
            let run = simulate_run(dx, dy, &area);
            if run.is_some() {
                vels_count += 1;
                if run.unwrap() > best_y {
                    best_y = run.unwrap();
                    best_vel = Some((dx, dy));
                }
            }
        }
    }

    if best_vel.is_none() {
        println!("No answer found in bound.");
    } else {
        println!("Best velocity: {:?}; achieves best y={}", best_vel.unwrap(), best_y);
        println!("Distinct initial velocities: {}", vels_count);
    }
}

fn simulate_run(init_dx: i32, init_dy: i32, area: &Rect) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut dx = init_dx;
    let mut dy = init_dy;

    let mut top_y = y;
    // println!(" -- sim: {},{} -- ", init_dx, init_dy);
    for _ in 0..BF_STEPS_MAX {
        // println!("x={}, y={}, dx={}, dy={}", x, y, dx, dy);
        // Check if in area
        if in_area(x, y, area) {
            return Some(top_y);
        }

        // Advance by a step
        x += dx;
        y += dy;
        dy -= 1;
        if dx > 0 {
            dx -= 1;
        } else if dx < 0 {
            dx += 1;
        }

        if y > top_y {
            top_y = y;
        }
    }

    None
}

fn in_area(x: i32, y: i32, area: &Rect) -> bool {
    x >= area.x1 && x <= area.x2 && y >= area.y1 && y <= area.y2
}
