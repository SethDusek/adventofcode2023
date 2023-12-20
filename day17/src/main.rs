use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

const INPUT: &'static str = include_str!("../input.txt");
const TEST: &'static str = include_str!("../test.txt");

#[derive(PartialOrd, Ord, Clone, Copy, Eq, Debug)]
struct Node {
    distance: Reverse<u32>,
    coords: (i32, i32),
    dir: (i32, i32),
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

fn adj(dir: (i32, i32)) -> [(i32, i32); 3] {
    match dir {
        (1, 0) => [(0, 1), (0, -1), (1, 0)],
        (-1, 0) => [(0, 1), (0, -1), (-1, 0)],
        (0, 1) => [(1, 0), (-1, 0), (0, 1)],
        (0, -1) => [(1, 0), (-1, 0), (0, -1)],
        _ => unreachable!(),
    }
}

struct Grid(Vec<u32>, usize);

impl Grid {
    fn get(&self, idx: (i32, i32)) -> Option<&u32> {
        if idx.0 < 0 || idx.1 < 0 || idx.0 as usize >= self.0.len() / self.1 || idx.1 as usize >= self.1 {
            None
        }
        else {
            self.0.get((idx.0 * self.1 as i32 + idx.1) as usize)
        }
    }
}

impl std::ops::Index<(i32, i32)> for Grid {
    type Output = u32;
    fn index(&self, index: (i32, i32)) -> &Self::Output {
        unsafe { self.get(index).unwrap_unchecked() }
        //self.get(index).unwrap()
    }
}

fn parse_grid(input: &str) -> Grid {
    let cols = unsafe { input.lines().next().unwrap_unchecked().len() } ;
    let grid = input
        .lines()
        .flat_map(|l| {
            l.chars()
                .map(move |c| c.to_digit(10).unwrap() as u32)
        })
        .collect::<Vec<u32>>();
    Grid(grid, cols)
}

fn mhd(a: (i32, i32), b: (i32, i32)) -> u32 {
    //((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
    0
}

fn run(grid: &Grid, min_steps: usize, max_steps: usize) {
    let dest = (
        (grid.0.len() / grid.1) as i32 - 1,
        grid.1 as i32 - 1
    );

    let mut heap = BinaryHeap::new();
    let mut dists = FxHashMap::default();
    let mut visited = FxHashSet::default();
    let node1 = Node {
        distance: Reverse(grid[(0, 1)] + mhd((0, 1), dest)),
        coords: (0, 1),
        dir: (0, 1),
        same_dir_count: 1,
    };
    heap.push(node1);
    dists.insert(node1, grid[(0, 1)]);
    let node2 = Node {
        distance: Reverse(grid[(1, 0)] + mhd((1, 0), dest)),
        coords: (1, 0),
        dir: (1, 0),
        same_dir_count: 1,
    };
    heap.push(node2);
    dists.insert(node2, grid[(1, 0)]);

    let mut visited_count = 0;
    while heap.len() != 0 {
        visited_count+=1;
        let cur = heap.pop().unwrap();
        if cur.coords == dest && cur.same_dir_count == min_steps {
            println!("Found dest! Heat loss: {:?}", cur.distance);
            println!("Visited {} nodes", visited_count);
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
            if grid.get(pos).is_none() {
                continue;
            }
            let same_dir_count = if adj == cur.dir {
                cur.same_dir_count + 1
            } else {
                1
            };
            let mut adj_node = Node {
                distance: Reverse(0), // needless optimization
                coords: pos,
                dir: adj,
                same_dir_count,
            };
            if visited.contains(&adj_node) {
                continue;
            }
            let cur_pos_dist = dists[&cur] + grid[pos];
            adj_node.distance.0 = cur_pos_dist + mhd(pos, dest);
            let dist = dists.entry(adj_node).or_insert(u32::MAX);
            if cur_pos_dist < *dist {
                *dist = cur_pos_dist;
                heap.push(adj_node);
            }
            if pos == (1, 0) || pos == (0, 1) {
                heap.push(adj_node);
            }
        }
   }
    //println!("{:?}", dists);
}

fn print_grid(grid: &FxHashMap<(i32, i32), u32>, prev: &FxHashMap<(i32, i32), (i32, i32)>) {
    let min_y = grid.keys().map(|(y, _)| *y).min().unwrap();
    let max_y = grid.keys().map(|(y, _)| *y).max().unwrap();
    let min_x: i32 = grid.keys().map(|(_, x)| *x).min().unwrap();
    let max_x = grid.keys().map(|(_, x)| *x).max().unwrap();
    let mut stack = FxHashMap::default();
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
