use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, BTreeMap},
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

fn run(grid: &Grid) {
    let end = ((grid.0.len() / grid.1) as i32 - 1, grid.1 as i32 - 2);
    dbg!(end);
    let mut heap = Vec::new();
    let mut visited = HashSet::new();
    let mut dists = HashMap::new();
    let start_node = Node {
        distance: 0,
        coords: (0, 1),
    };
    heap.push(start_node);
    dists.insert(start_node, 0);
    let mut max_dist = 0;
    while heap.len() != 0 {
        let cur = heap.pop().unwrap();
        //dbg!(cur);

        visited.insert(cur.coords);
        if cur.coords == end {
            max_dist = std::cmp::max(max_dist, cur.distance);
            visited.remove(&cur.coords);
        }
        let mut adj = Vec::new();
        match grid[cur.coords] {
            b'^' => adj.push((-1, 0)),
            b'v' => adj.push((1, 0)),
            b'>' => adj.push((0, 1)),
            b'<' => adj.push((0, -1)),
            _ => {
                let adjacent_nodes = [(0, -1), (0, 1), (1, 0), (-1, 0)];
                for adjacent in adjacent_nodes {
                    adj.push(adjacent);
                }
            }
        }
        for adj in adj {
            let adj_dist = dists[&cur] + 1;
            let node = Node {
                distance: adj_dist,
                coords: (cur.coords.0 + adj.0, cur.coords.1 + adj.1),
            };
            if grid.get(node.coords).is_none()
                || grid[node.coords] == b'#'
                || visited.contains(&node.coords)
            {
                continue;
            }
            //if dists.get(&node).copied().unwrap_or(0) < adj_dist {
            dists.insert(node, adj_dist);
            heap.push(node);
            //}
        }
    }
    if visited.contains(&(22, 22)) {
        panic!();
    }
    print_grid(&grid, &visited);
    println!("{}", max_dist);
}

fn print_grid(grid: &Grid, visited: &HashSet<(i32, i32)>) {
    for y in 0..grid.0.len() / grid.1 {
        for x in 0..grid.1 {
            if visited.contains(&(y as i32, x as i32)) {
                print!("O");
            } else {
                print!("{}", grid[(y as i32, x as i32)] as char);
            }
        }
        println!("");
    }
}

fn dfs1(
    grid: &Grid,
    cur_node: (i32, i32),
    goal_node: (i32, i32),
    dist: usize,
    visited: &mut HashSet<(i32, i32)>,
    memo: &mut HashMap<((i32, i32), (i32, i32)), (usize, HashSet<(i32, i32)>)>,
    prev_node: (i32, i32),
) -> usize {
    if cur_node == goal_node {
        return dist;
    }
    // if let Some((ans, hs)) = memo.get(&(prev_node, cur_node)) {
    //     let adjacent_nodes = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    //     let adjacent_count = adjacent_nodes
    //         .iter()
    //         .map(|adj| (cur_node.0 + adj.0, cur_node.1 + adj.1))
    //         .filter(|&node| !grid.get(node).is_none() && grid[node] != b'#')
    //         .count();
    //     //println!("{} {}", adjacent_count, hs.len());
    //     if hs.len() == adjacent_count {
    //         visited.insert(cur_node);
    //         return *ans;
    //     }
    // }

    visited.insert(cur_node);

    let mut adj = Vec::new();
    match grid[cur_node] {
        // b'^' => adj.push((-1, 0)),
        // b'v' => adj.push((1, 0)),
        // b'>' => adj.push((0, 1)),
        // b'<' => adj.push((0, -1)),
        _ => {
            let adjacent_nodes = [(0, -1), (0, 1), (1, 0), (-1, 0)];
            for adjacent in adjacent_nodes {
                adj.push(adjacent);
            }
        }
    }
    let mut max_dist = 0;
    for adj in adj {
        let adj = (cur_node.0 + adj.0, cur_node.1 + adj.1);
        if grid.get(adj).is_none() || grid[adj] == b'#' || visited.contains(&adj) {
            continue;
        }
        let mut visited = visited.clone();
        max_dist = std::cmp::max(
            max_dist,
            dfs1(grid, adj, goal_node, dist + 1, &mut visited, memo, cur_node),
        );
    }
    if cur_node != (0, 1) {
        memo.entry((prev_node, cur_node))
            .and_modify(|v| {
                v.0 = std::cmp::max(v.0, max_dist);
                v.1.insert(prev_node);
            })
            .or_insert((max_dist, HashSet::from_iter(std::iter::once(prev_node))));
    }
    max_dist
}

fn root_tree(
    grid: &Grid,
    root: (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
    tree: &mut HashMap<(i32, i32), Vec<(i32, i32)>>,
) {
    visited.insert(root);
    let adj = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    tree.entry(root).or_insert(vec![]);
    for adj in adj {
        let adj = (root.0 + adj.0, root.1 + adj.1);
        if grid.get(adj).is_none() || grid[adj] == b'#' || visited.contains(&adj) {
            continue;
        }
        tree.entry(root)
            .and_modify(|v| v.push(adj))
            .or_insert(vec![adj]);
        root_tree(grid, adj, visited, tree);
    }
}

fn dfs2(
    tree: &HashMap<(i32, i32), Vec<(i32, i32)>>,
    cur_node: (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
    finish_order: &mut Vec<(i32, i32)>,
) {
    visited.insert(cur_node);
    for adj in &tree[&cur_node] {
        if visited.contains(&adj) {
            continue;
        }
        dfs2(tree, *adj, visited, finish_order);
    }
    finish_order.push(cur_node);
}

// start node must be a junction
fn dfs3(grid: &Grid, start_node: (i32, i32)) -> BTreeMap<(i32, i32), HashMap<(i32, i32), usize>> {
    let mut hm = BTreeMap::new();
    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    stack.push((start_node, 0, start_node, visited));
    while stack.len() != 0 {
        let (cur_node, mut dist, prev_junction_node, mut visited) = stack.pop().unwrap();
        visited.insert(cur_node);
        let (adj, is_junction) = is_junction(cur_node, grid, &visited);
        if is_junction {
            hm.entry(prev_junction_node)
                .and_modify(|hm: &mut HashMap<(i32, i32), usize>| {
                    //assert!(!hm.contains_key(&cur_node));
                    hm.insert(cur_node, dist);
                })
                .or_insert(std::iter::once((cur_node, dist)).collect());
            hm.entry(cur_node)
                .and_modify(|hm: &mut HashMap<(i32, i32), usize>| {
                    hm.insert(prev_junction_node, dist);
                })
                .or_insert(std::iter::once((prev_junction_node, dist)).collect());
            dist = 0;
        }
        for adj in adj
            .iter()
            .map(|adj| (cur_node.0 + adj.0, cur_node.1 + adj.1))
            .filter(|&adj| grid.get(adj).is_some()  && grid[adj] != b'#')
            .filter(|node| !visited.contains(node))
            .filter(|&node| node != cur_node) {
                stack.push((adj, dist + 1, if is_junction && cur_node != prev_junction_node { continue; } else { prev_junction_node }, visited.clone()));
        }
    }

    hm
}

fn is_junction(cur_node: (i32, i32), grid: &Grid) -> ([(i32, i32); 4], bool) {
    let adj = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    let is_junction = adj
        .iter()
        .map(|adj| (cur_node.0 + adj.0, cur_node.1 + adj.1))
        .filter(|&adj| grid.get(adj).is_some()  && grid[adj] != b'#')
        .filter(|&node| node != cur_node)
        .count()
        > 1;
    (adj, is_junction)
}
fn main() {
    let grid = parse_grid(INPUT);
    let graph = dfs3(&grid, (1, 1));
    println!("{} {:?}\n{:?}", graph.len(), graph, graph.keys().copied().enumerate().collect::<Vec<(usize, (i32, i32))>>());
    let mut last = ((grid.0.len() / grid.1) as i32 - 1, grid.1 as i32 - 2);
    let mut visited = HashSet::new();
    let mut memo = HashMap::new();
    return;
    println!(
        "{}",
        dfs1(&grid, (1, 1), last, 1, &mut visited, &mut memo, (0, 1))
    );


    let mut tree = HashMap::new();
    root_tree(&grid, (0, 1), &mut visited, &mut tree);
    println!("{:?}", tree);
    let mut visited = HashSet::new();
    let mut finish_order = Vec::new();

    //dfs2(&tree, (0, 1), &mut visited, &mut finish_order);
    for node in tree.iter() {
        if !visited.contains(node.0) {
            dfs2(&tree, *node.0, &mut visited, &mut finish_order);
        }
    }

    let mut parent = HashMap::new();
    let mut dist = HashMap::new();
    dist.insert((0, 1), 0);
    while finish_order.len() != 0 {
        let cur = finish_order.pop().unwrap();
        for child in &tree[&cur] {
            let child_dist = dist.get(child).copied().unwrap_or(0);
            if dist[&cur] + 1 > child_dist {
                dist.insert(*child, dist[&cur] + 1);
                parent.insert(*child, cur);
            }
        }
    }
    let mut count = 0;
    while parent.get(&last).is_some() {
        last = parent[&last];
        count += 1;
    }
    println!("{}", count);
}
