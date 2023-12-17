use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

const INPUT: &'static str = include_str!("../input.txt");
const TEST: &'static str = include_str!("../test.txt");

#[derive(PartialOrd, Ord, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    distance: Reverse<u64>,
    coords: (i64, i64),
    dir: (i64, i64),
    same_dir_count: usize,
}

fn adj(dir: (i64, i64)) -> [(i64, i64); 3] {
    match dir {
        (1, 0) => [(0, 1), (0, -1), (1, 0)],
        (-1, 0) => [(0, 1), (0, -1), (-1, 0)],
        (0, 1) => [(1, 0), (-1, 0), (0, 1)],
        (0, -1) => [(1, 0), (-1, 0), (0, -1)],
        _ => unreachable!(),
    }
}

fn run(input: &str) {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i64, j as i64), c.to_digit(10).unwrap() as u64))
        })
        .collect::<HashMap<(i64, i64), u64>>();
    let dest = (
        grid.keys().map(|(i, j)| *i).max().unwrap(),
        grid.keys().map(|(_, j)| *j).max().unwrap(),
    );

    let mut heap = BinaryHeap::new();
    let mut dists = HashMap::new();
    let mut visited = HashSet::new();
    let mut prevs: HashMap<(i64, i64), (i64, i64)> = HashMap::new();
    // dists.insert((0, 0), 0);
    // dists.insert((0, 1), grid[&(0, 1)]);
    // dists.insert((1, 0), grid[&(1, 0)]);
    // prevs.insert((0, 1), (0, 0));
    // prevs.insert((1, 0), (0, 0));
    let node1 = Node {
        distance: Reverse(grid[&(0, 1)]),
        coords: (0, 1),
        dir: (0, 1),
        same_dir_count: 1,
    };
    heap.push(node1);
    dists.insert(node1, node1.distance.0);
    let node2 = Node {
        distance: Reverse(grid[&(1, 0)]),
        coords: (1, 0),
        dir: (1, 0),
        same_dir_count: 1,
    };
    heap.push(node2);
    dists.insert(node2, node2.distance.0);

    // heap.push(Node {
    //     distance: Reverse(grid[&(1, 0)]),
    //     coords: (1, 0),
    //     dir: (1, 0),
    //     same_dir_count: 1
    // });
    // heap.push(Node {
    //     distance: Reverse(0),
    //     coords: (0, 0),
    //     dir: (1, 0),
    //     same_dir_count: 1
    // });
    while heap.len() != 0 {
        let cur = heap.pop().unwrap();
        if cur.coords == dest {
            println!("Found dest! Heat loss: {:?}", cur.distance);
            //print_grid(&grid, &prevs);
            break;
            //panic!();
        }
        visited.insert((cur.coords, cur.dir, cur.same_dir_count));
        for adj in adj(cur.dir) {
            if adj == cur.dir && cur.same_dir_count >= 3 {
                continue;
            }
            let pos = (cur.coords.0 + adj.0, cur.coords.1 + adj.1);
            if !grid.contains_key(&pos) {
                continue;
            }
            let same_dir_count = if adj == cur.dir {
                cur.same_dir_count + 1
            } else {
                1
            };
            if same_dir_count > 3 {
                panic!();
            }
            let adj_node = Node {
                distance: Reverse(dists[&cur] + grid[&pos]),
                coords: pos,
                dir: adj,
                same_dir_count,
            };
            if visited.contains(&(adj_node.coords, adj_node.dir, adj_node.same_dir_count)) {
                continue;
            }
            if dists[&cur] + grid[&pos] < dists.get(&adj_node).copied().unwrap_or(u64::MAX) {
                dists.insert(adj_node, dists[&cur] + grid[&pos]);
                heap.push(adj_node);
                prevs.insert(pos, cur.coords);
            }
        }
    }
    //println!("{:?}", dists);
}
fn print_grid(grid: &HashMap<(i64, i64), u64>, prev: &HashMap<(i64, i64), (i64, i64)>) {
    let min_y = grid.keys().map(|(y, _)| *y).min().unwrap();
    let max_y = grid.keys().map(|(y, _)| *y).max().unwrap();
    let min_x: i64 = grid.keys().map(|(_, x)| *x).min().unwrap();
    let max_x = grid.keys().map(|(_, x)| *x).max().unwrap();
    let mut stack = HashMap::new();
    let mut cur = (max_y, max_x);
    while prev.get(&cur).is_some() {
        let prev_node = prev.get(&cur).unwrap();
        let diff = (cur.0 - prev_node.0, cur.1 - prev_node.1);
        let ch = match diff {
            (-1, 0) => '^',
            (1, 0) => 'v',
            (0, 1) => '>',
            (0, -1) => '<',
            _ => unreachable!(),
        };
        stack.insert(cur, ch);
        cur = prev[&cur];
    }
    for i in min_y..=max_y {
        for j in min_x..=max_x {
            if let Some(ch) = stack.get(&(i, j)) {
                print!("{ch}")
            } else {
                print!("{}", grid.get(&(i, j)).unwrap());
            }
        }
        print!("\n");
    }
}

fn main() {
    run(TEST);
    run(INPUT);
}
