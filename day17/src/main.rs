use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

const INPUT: &'static str = include_str!("../input.txt");
const TEST: &'static str = include_str!("../test.txt");

#[derive(PartialOrd, Ord, Clone, Copy, Eq, Debug)]
struct Node {
    distance: Reverse<u64>,
    coords: (i64, i64),
    dir: (i64, i64),
    same_dir_count: usize,
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coords.hash(state);
        self.dir.hash(state);
        self.same_dir_count.hash(state);
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords
            && self.dir == other.dir
            && self.same_dir_count == other.same_dir_count
    }
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

struct Grid(Vec<u64>, usize);

impl Grid {
    fn get(&self, idx: (i64, i64)) -> Option<&u64> {
        if idx.0 < 0 || idx.1 < 0 || idx.0 as usize >= self.0.len() / self.1 || idx.1 as usize >= self.1 {
            None
        }
        else {
            self.0.get((idx.0 * self.1 as i64 + idx.1) as usize)
        }
    }
}

impl std::ops::Index<(i64, i64)> for Grid {
    type Output = u64;
    fn index(&self, index: (i64, i64)) -> &Self::Output {
        self.get(index).unwrap()
    }
}

fn parse_grid(input: &str) -> Grid {
    let cols = input.lines().next().unwrap().len();
    let grid = input
        .lines()
        .flat_map(|l| {
            l.chars()
                .map(move |c| c.to_digit(10).unwrap() as u64)
        })
        .collect::<Vec<u64>>();
    Grid(grid, cols)
}

fn run(grid2: &Grid, min_steps: usize, max_steps: usize) {
    let dest = (
        (grid2.0.len() / grid2.1) as i64 - 1,
        grid2.1 as i64 - 1
    );

    let mut heap = BinaryHeap::new();
    let mut dists = HashMap::new();
    let mut visited = HashSet::new();
    let node1 = Node {
        distance: Reverse(grid2[(0, 1)]),
        coords: (0, 1),
        dir: (0, 1),
        same_dir_count: 1,
    };
    heap.push(node1);
    dists.insert(node1, node1.distance.0);
    let node2 = Node {
        distance: Reverse(grid2[(1, 0)]),
        coords: (1, 0),
        dir: (1, 0),
        same_dir_count: 1,
    };
    heap.push(node2);
    dists.insert(node2, node2.distance.0);

    while heap.len() != 0 {
        let cur = heap.pop().unwrap();
        if cur.coords == dest {
            println!("Found dest! Heat loss: {:?}", cur.distance);
            break;
        }
        visited.insert(cur);
        for adj in adj(cur.dir) {
            if adj == cur.dir && cur.same_dir_count >= max_steps {
                continue;
            }
            if adj != cur.dir && cur.same_dir_count < min_steps {
                continue;
            }
            let pos = (cur.coords.0 + adj.0, cur.coords.1 + adj.1);
            if grid2.get(pos).is_none() {
                continue;
            }
            let same_dir_count = if adj == cur.dir {
                cur.same_dir_count + 1
            } else {
                1
            };
            let cur_pos_dist = dists[&cur] + grid2[pos];
            let adj_node = Node {
                distance: Reverse(cur_pos_dist),
                coords: pos,
                dir: adj,
                same_dir_count,
            };
            if visited.contains(&adj_node) {
                continue;
            }
            let dist = dists.entry(adj_node).or_insert(u64::MAX);
            if cur_pos_dist < *dist {
                *dist = cur_pos_dist;
                heap.push(adj_node);
            }
        }
        while heap
            .peek()
            .map(|top| visited.contains(&top))
            .unwrap_or(false)
        {
            dbg!(heap.pop());
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
    let test_grid = parse_grid(TEST);
    let input_grid = parse_grid(INPUT);
    run(&test_grid, 0, 3);
    run(&input_grid, 0, 3);

    run(&test_grid, 4, 10);
    run(&input_grid, 4, 10);
}
