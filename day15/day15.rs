use std::env;
use std::fs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone)]
struct Edge {
    v: usize,
    w: u32,
}
type Graph = Vec<Vec<Edge>>;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{} <input.txt>", &args[0]);
        std::process::exit(1);
    }

    let file = fs::read(&args[1]).expect("Could not read input file given.");
    let data = String::from_utf8_lossy(&file);

    let n = data.lines().next().unwrap().len();
    let mut structure = vec![vec![0; n]; n];
    for (i, line) in data.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            structure[i][j] = c.to_digit(10).unwrap();
        }
    }

    let part1_graph = input_to_graph(&structure);
    println!("Part 1: {}", dijkstra(&part1_graph));

    // Build part 2 structure (reuse/replace existing)
    // First extend the first n lines
    for i in 0..n {
        for j in 0..5*n {
            let mut p = structure[i][j] + 1;
            if p > 9 { p -= 9; }
            structure[i].push(p);
        }
    }
    // Then extend the rows by copying down
    for i in n..5*n {
        let p = structure[i-n].iter().map(|x| {
            let mut xx = x + 1;
            if xx > 9 { xx -= 9; }
            xx
        }).collect::<Vec<u32>>();
        structure.push(p);
    }

    let part2_graph = input_to_graph(&structure);
    println!("Part 2: {}", dijkstra(&part2_graph));
}

fn input_to_graph(structure: &Vec<Vec<u32>>) -> Graph {
    let n = structure.len();
    let mut graph: Graph = vec![Vec::new(); n*n];
    for i in 0..n {
        for j in 0..n {
            let graph_i = i*n+j;

            if j < n - 1 {
                graph[graph_i].push(Edge { v: graph_i + 1, w: structure[i][j+1] });
            }

            if i < n - 1 {
                graph[graph_i].push(Edge { v: graph_i + n, w: structure[i+1][j] });
            }

            if j > 0 {
                graph[graph_i].push(Edge { v: graph_i - 1, w: structure[i][j-1] });
            }

            if i > 0 {
                graph[graph_i].push(Edge { v: graph_i - n, w: structure[i-1][j] });
            }
        }
    }

    graph
}

#[derive(Eq, PartialEq)]
struct QueueVertex {
    i: usize,
    d: u32,
}

impl Ord for QueueVertex {
    fn cmp(&self, other: &Self) -> Ordering {
        // From https://doc.rust-lang.org/std/collections/binary_heap/index.html
        other.d.cmp(&self.d).then_with(|| self.d.cmp(&other.d))
    }
}

impl PartialOrd for QueueVertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: &Graph) -> u32 {
    let n = graph.len();
    let mut dist = vec![u32::MAX; n];
    dist[0] = 0;

    let mut q = BinaryHeap::new();
    for i in 0..n {
        q.push(QueueVertex { i, d: dist[i] });
    }

    while !q.is_empty() {
        let u = q.pop().unwrap();

        // We don't remove the old things still in the queue as it is inconvenient
        // So if found a better way, just drop it
        if dist[u.i] < u.d  {
            continue;
        }

        for e in &graph[u.i] {
            let new_dist = dist[u.i] + e.w;
            if new_dist < dist[e.v] {
                dist[e.v] = new_dist;
                q.push(QueueVertex { i: e.v, d: new_dist });
            }
        }
    }

    dist[n-1]
}
