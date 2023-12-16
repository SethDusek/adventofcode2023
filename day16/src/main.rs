const INPUT: &'static str = include_str!("../input.txt");
const TEST: &'static str = include_str!("../test.txt");
use std::{collections::{HashMap, HashSet}, str::FromStr, cmp};

fn bounce(ray_dir: (i64, i64), mirror: char) -> Vec<(i64, i64)> {
    //dbg!(mirror);
    match mirror {
        '.' => vec![ray_dir],
        '\\' if ray_dir == (0, 1) => vec![(1, 0)],
        '/' if ray_dir == (0, 1) => vec![(-1, 0)],
        '\\' if ray_dir == (0, -1) => vec![(-1, 0)],
        '/' if ray_dir == (0, -1) => vec![(1, 0)],

        '\\' if ray_dir == (1, 0) => vec![(0, 1)],
        '\\' if ray_dir == (-1, 0) => vec![(0, -1)],
        '/' if ray_dir == (1, 0) => vec![(0, -1)],
        '/' if ray_dir == (-1, 0) => vec![(0, 1)],
        '|' if ray_dir.1 != 0 => vec![(1, 0), (-1, 0)],
        '-' if ray_dir.0 != 0 => vec![(0, 1), (0, -1)],
        '|' | '-' => vec![ray_dir],
        _ => unreachable!(),
    }
}

fn parse_grid(input: &str) -> HashMap<(i64, i64), char> {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i64, j as i64), c))
        })
        .collect::<HashMap<(i64, i64), char>>();
    grid
}
fn run(grid: &HashMap<(i64, i64), char>, initial_pos: (i64, i64), initial_dir: (i64, i64)) -> usize {
    let mut energized = HashSet::new();
    let mut pos_dir = vec![(initial_pos, initial_dir)];
    energized.insert(pos_dir[0].0);
    let mut visited = HashSet::new();
    while pos_dir.len() != 0 {
        let cur = pos_dir.pop().unwrap();
        visited.insert(cur);
        if grid.get(&cur.0).is_none() {
            continue;
        }
        energized.insert(cur.0);
        let dirs = bounce(cur.1, *grid.get(&cur.0).unwrap());
        for dir in dirs {
            let new_pos =
             (cur.0.0 + dir.0, cur.0.1 + dir.1);
            if visited.contains(&(new_pos, dir)) {
                continue;
            }
            if grid.get(&new_pos).is_none() {
                continue;
            }
            pos_dir.push((new_pos, dir));
        }
    }
    energized.len()
}

fn print_grid(grid: &HashMap<(i64, i64), char>, energized: &HashSet<(i64, i64)>) {
    let min_y = grid.keys().map(|(y, _)| *y).min().unwrap();
    let max_y = grid.keys().map(|(y, _)| *y).max().unwrap();
    let min_x: i64 = grid.keys().map(|(_, x)| *x).min().unwrap();
    let max_x = grid.keys().map(|(_, x)| *x).max().unwrap();
    for i in min_y..=max_y {
        for j in min_x..=max_x {
            if energized.contains(&(i, j)) {
                print!("#")
            }
            else {
                print!("{}", grid.get(&(i, j)).unwrap());
            }
        }
        print!("\n");
    }
}

fn run_parts(grid: &HashMap<(i64, i64), char>) {
    let min_y = 0;
    let max_y = grid.keys().map(|(y, _)| *y).max().unwrap();
    let min_x: i64 = 0;
    let max_x = grid.keys().map(|(_, x)| *x).max().unwrap();
    let mut ans = 0;
    println!("Part 1: {}", run(&grid, (0, 0), (0, 1)));
    for x in min_x..=max_x {
        ans = cmp::max(ans, run(&grid, (min_y, x), (1, 0)));
        ans = cmp::max(ans, run(&grid, (max_y, x), (-1, 0)));
    }
    for y in min_y..=max_y {
        ans = cmp::max(ans, run(&grid, (y, min_x), (0, 1)));
        ans = cmp::max(ans, run(&grid, (y, max_x), (0, -1)));
    }
    println!("Part 2: {ans}");
}
fn main() {
    let grid = parse_grid(TEST);
    run_parts(&grid);
    let grid = parse_grid(INPUT);
    run_parts(&grid);


    //run(&grid, (0, 0), (0, 1));
}
