use std::env;
use std::fs;

// use ODD so theres a midpoint
const IMAGE_SIZE: usize = 150;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{} <input.txt>", &args[0]);
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);
    let mut lines = data.lines();

    let enhance = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<bool>>();
    lines.next();

    let mut image = [[false; IMAGE_SIZE]; IMAGE_SIZE];
    let input = lines
        .map(|r| r.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    // copy the input into the centre of the image
    let height = input.len();
    let width = input[0].len();
    let start_h = IMAGE_SIZE / 2 - height / 2;
    let start_w = IMAGE_SIZE / 2 - width / 2;
    for r in 0..height {
        for c in 0..width {
            image[start_h + r][start_w + c] = input[r][c];
        }
    }

    // dump_image(&image);
    let mut back_image;
    for i in 0..2 { // run twice
        back_image = image.clone();
        for r in 1..IMAGE_SIZE-1 { // 1.. won't work properly if image hits the edge, but oh well
            for c in 1..IMAGE_SIZE-1 {
                let mut v = 0;
                for (r2, c2) in sweep_3x3(r, c) {
                    v <<= 1;
                    if image[r2][c2] {
                        v += 1;
                    }
                }
                // println!("r={},c={} -> v={}", r,c,v);
                back_image[r][c] = enhance[v];
            }
        }
        image = back_image;

        // because we are skipping edges, need to account for it IF 0=light up
        if enhance[0] {
            let v = if i % 2 == 0 { true } else { false };
            for c in 0..IMAGE_SIZE {
                image[0][c] = v;
                image[IMAGE_SIZE-1][c] = v;
            }
            for r in 1..IMAGE_SIZE-1 {
                image[r][0] = v;
                image[r][IMAGE_SIZE-1] = v;
            }
        }
        // dump_image(&image);
    }

    println!("Part 1 lit count: {}", image.iter().flatten().filter(|&x| *x).count());
}

fn sweep_3x3(r: usize, c: usize) -> [(usize, usize); 9] {
    [
        (r - 1, c - 1),
        (r - 1, c    ),
        (r - 1, c + 1),
        (r    , c - 1),
        (r    , c    ),
        (r    , c + 1),
        (r + 1, c - 1),
        (r + 1, c    ),
        (r + 1, c + 1),
    ]
}

#[allow(dead_code)]
fn dump_image(image: &[[bool; IMAGE_SIZE]; IMAGE_SIZE]) {
    for row in image {
        for x in row {
            print!("{}", if *x { "#" } else { "." });
        }
        print!("\n");
    }
    print!("\n");
}
