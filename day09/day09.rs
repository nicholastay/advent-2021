use std::env;
use std::fs;

type Map = Vec<Vec<u32>>;
type Point = (usize, usize);

// Returns lowest positions
fn part1(map: &Map) -> Vec<Point> {
    let mut low_points: Vec<Point> = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            let adjacents = adjacent_points(map, (i, j));
            if adjacents.iter().all(|&(r, c)| height < &map[r][c]) {
                low_points.push((i, j));
            }
        }
    }

    low_points
}

fn adjacent_points(map: &Map, (r, c): Point) -> Vec<Point> {
    let h = map.len();
    let w = map[0].len();

    let mut adjacents: Vec<Point> = Vec::new();
    if r > 0 {
        adjacents.push((r - 1, c));
    }
    if r < h - 1 {
        adjacents.push((r + 1, c));
    }
    if c > 0 {
        adjacents.push((r, c - 1));
    }
    if c < w - 1 {
        adjacents.push((r, c + 1));
    }

    adjacents
}

// Returns 3 largest basin sizes multiplied
fn part2(map: &Map) -> u32 {
    // println!("{}", part2_basin_size(map, (0, 9), &mut Vec::new()));

    let mut visited = Vec::new();
    let mut sizes = (0..map.len())
        .flat_map(|i| {
            (0..map[0].len())
                .map(|j| part2_basin_size(map, (i, j), &mut visited))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<u32>>();
    sizes.sort_unstable();

    sizes.into_iter().rev().take(3).product::<u32>()
}

// There probably is a more elegant way than a visited vec...
// Although, it lets us share the vector between executions, to avoid double counting
// Each square is uniquely allocated to a single basin.
fn part2_basin_size(map: &Map, point: Point, visited: &mut Vec<Point>) -> u32 {
    let (r, c) = point;
    if map[r][c] == 9 || visited.contains(&point) {
        return 0;
    }

    // println!("counting {:?}", point);
    visited.push(point);

    1 + adjacent_points(map, point)
        .iter()
        .map(|p| part2_basin_size(map, *p, visited))
        .sum::<u32>()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{} <input.txt>", &args[0]);
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    let map = data
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let low_points = part1(&map);
    println!("Part 1 low points: {:?}", low_points);
    println!(
        "Part 1 risk level sum: {}",
        low_points.iter().map(|&(r, c)| 1 + map[r][c]).sum::<u32>(),
    );

    println!("");
    println!("Part 2 largest basin sizes multiplied: {}", part2(&map));
}
