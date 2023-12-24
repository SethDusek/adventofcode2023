use std::{
    cmp::{Ordering, Reverse},
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
    hash::Hash,
};
const TEST: &'static str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
const INPUT: &'static str = include_str!("../input.txt");
struct Grid(Vec<u8>, usize);

#[derive(PartialOrd, Ord, Clone, Copy, Eq, Debug)]
struct Node {
    distance: u32,
    coords: (i32, i32),
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coords.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords
    }
}

impl Grid {
    fn get(&self, idx: (i32, i32)) -> Option<&u8> {
        if idx.0 < 0
            || idx.1 < 0
            || idx.0 as usize >= self.0.len() / self.1
            || idx.1 as usize >= self.1
        {
            None
        } else {
            self.0.get((idx.0 * self.1 as i32 + idx.1) as usize)
        }
    }
}

impl std::ops::Index<(i32, i32)> for Grid {
    type Output = u8;
    fn index(&self, index: (i32, i32)) -> &Self::Output {
        //unsafe { self.get(index).unwrap_unchecked() }
        self.get(index).unwrap()
    }
}

fn parse_grid(input: &str) -> Grid {
    let cols = unsafe { input.lines().next().unwrap_unchecked().len() };
    let grid = input
        .lines()
        .flat_map(|l| l.bytes().map(move |c| c))
        .collect::<Vec<u8>>();
    Grid(grid, cols)
}

fn adjacent(cur_node: (i32, i32), grid: &Grid) -> impl Iterator<Item = (i32, i32)> + '_ {
    const ADJ: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    ADJ.iter()
        .map(move |adj| (cur_node.0 + adj.0, cur_node.1 + adj.1))
        .filter(|&adj| grid.get(adj).is_some() && grid[adj] != b'#')
}
fn is_junction(cur_node: (i32, i32), grid: &Grid) -> bool {
    let is_junction = adjacent(cur_node, grid).count() > 2;
    grid.get(cur_node).is_some() && grid[cur_node] != b'#' && is_junction
}

fn find_edges(cur_junction: (i32, i32), grid: &Grid, graph: &mut HashMap<(i32, i32), HashMap<(i32, i32), usize>>) {
    let mut stack = Vec::new();
    stack.push((cur_junction, 0));
    let mut visited = HashSet::new();
    while stack.len() != 0 {
        let cur = stack.pop().unwrap();
        visited.insert(cur.0);
        if is_junction(cur.0, grid) && cur.0 != cur_junction {
            if let Some(dist) = graph.get(&cur_junction).and_then(|hm| hm.get(&cur.0)) {
                if *dist < cur.1 {
                    panic!();
                }
            }
            graph.entry(cur_junction).or_insert(HashMap::new()).insert(cur.0, cur.1);
            graph.entry(cur.0).or_insert(HashMap::new()).insert(cur_junction, cur.1);
        }
        else {
            for adj in adjacent(cur.0, grid).filter(|node| !visited.contains(node)) {
                stack.push((adj, cur.1 + 1));
            }
        }
    }
}
fn dfsver4finalforsure(cur_node: (i32, i32), goal_node: (i32, i32), graph: &HashMap<(i32, i32), HashMap<(i32, i32), usize>>, visited: &mut HashSet<(i32, i32)>, steps: usize) -> usize {
    if cur_node == goal_node {
        return steps;
    }
    visited.insert(cur_node);

    let mut max_dist = 0;
    for (&adj, &dist) in &graph[&cur_node] {
        if visited.contains(&adj) {
            continue;
        }
        max_dist = std::cmp::max(max_dist, dfsver4finalforsure(adj, goal_node, graph, visited, steps + dist));
        visited.remove(&adj);
    }
    max_dist
}

fn find_first_junction(cur_node: (i32, i32), grid: &Grid, junctions: &HashSet<(i32, i32)>) -> ((i32, i32), usize) {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    stack.push((cur_node, 0));
    while stack.len() != 0 {
        let cur = stack.pop().unwrap();
        if junctions.contains(&cur.0) {
            return cur;
        }
        visited.insert(cur.0);
        for adj in adjacent(cur.0, grid) {
            if visited.contains(&adj) {
                continue;
            }
            stack.push((adj, cur.1 + 1));
        }
    }
    unreachable!()
}
fn main() {
    let grid = parse_grid(INPUT);
    let junctions = (0..(grid.0.len() / grid.1) as i32)
        .flat_map(|y| (0..grid.1 as i32).map(move |x| (y, x)))
        .filter(|&node| is_junction(node, &grid))
        .collect::<HashSet<(i32, i32)>>();
    // TODO: build graph for *all* junctions not each
    let mut graph = HashMap::new();
    for junction in &junctions {
        find_edges(*junction, &grid, &mut graph);
    }
    println!("{} {:?}", junctions.len(), junctions);
    println!("{:?}", graph);
    let (start, start_dist) = find_first_junction((0, 1), &grid, &junctions);
    let (end, end_dist) = find_first_junction(((grid.0.len() / grid.1) as i32 - 1, grid.1 as i32 - 2), &grid, &junctions);
    println!("{start:?} {start_dist}");
    println!("{end:?} {end_dist}");
    let mut visited = HashSet::new();
    println!("{}", dfsver4finalforsure(start, end, &graph, &mut visited, 0) + start_dist + end_dist);
    //println!("{}", dfsver4finalforsure((5, 3), (19, 19), &graph, &mut visited, 0));
}
