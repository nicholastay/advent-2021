use std::env;
use std::fs;

const MATRIX_SIZE: usize = 1000;

type Matrix = [[PointState; MATRIX_SIZE]; MATRIX_SIZE];

#[derive(PartialEq, Clone, Copy)]
enum PointState {
    Unmarked,
    One,
    Marked,
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Supply input file.");
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    let mut p1_matrix: Matrix = [[PointState::Unmarked; MATRIX_SIZE]; MATRIX_SIZE];
    let mut p1_count: i32 = 0;

    let mut p2_matrix: Matrix = [[PointState::Unmarked; MATRIX_SIZE]; MATRIX_SIZE];
    let mut p2_count: i32 = 0;

    for line in data.lines() {
        let mut splits = line.split(" -> ");

        let from: Point = str_to_point(splits.next().unwrap());
        let to: Point = str_to_point(splits.next().unwrap());

        println!("Marking from {:?} to {:?}", from, to);
        mark_line(true, &mut p1_matrix, &mut p1_count, &from, &to);
        mark_line(false, &mut p2_matrix, &mut p2_count, &from, &to);
    }

    println!("");
    println!("Part 1 final count: {}", p1_count);
    println!("Part 2 final count: {}", p2_count);
}

fn str_to_point(coords: &str) -> Point {
    let mut points = coords.split(",");
    Point {
        x: points.next().unwrap().parse().unwrap(),
        y: points.next().unwrap().parse().unwrap(),
    }
}

fn mark_line(part_one: bool, mat: &mut Matrix, count: &mut i32, from: &Point, to: &Point) {
    // Only support horizontal and vertical lines for Part 1.
    if part_one && from.x != to.x && from.y != to.y {
        //println!("WARN: Non horizontal and non vertical lines are not supported in Part 1. Skipping {:?} -> {:?}.", from, to);
        return
    }

    let mut point: Point = *from;

    loop {
        //println!("-- Marking {:?}", point);
        let mat_state: &mut PointState = &mut mat[point.x][point.y];
        if *mat_state == PointState::Unmarked {
            // Now it has one line
            *mat_state = PointState::One;
        } else if *mat_state == PointState::One {
            // Count it, it has now been >=2 lines crossed
            *count += 1;
            *mat_state = PointState::Marked;
        }
        // Otherwise, if marked, don't do any counting/changing

        // Count the last one
        if point == *to {
            break;
        }

        if from.x == to.x {
            if to.y < from.y {
                point.y -= 1;
            } else {
                point.y += 1;
            }
        } else if from.y == to.y {
            if to.x < from.x {
                point.x -= 1;
            } else {
                point.x += 1;
            }
        } else {
            // We are assuming these are 45deg, infinite loop if not...
            if to.x < from.x && to.y < from.y {
                point.x -= 1;
                point.y -= 1;
            } else if to.x < from.x && to.y > from.y {
                point.x -= 1;
                point.y += 1
            } else if to.x > from.x && to.y > from.y {
                point.x += 1;
                point.y += 1
            } else if to.x > from.x && to.y < from.y {
                point.x += 1;
                point.y -= 1
            } else {
                println!("unreachable");
                std::process::exit(1);
            }
        }
    }
}
