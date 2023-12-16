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

fn parse_grid(input: &str) -> (Vec<char>, usize) {
    let mut cols = input.lines().next().unwrap().len();
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(move |(j, c)| c)
        })
        .collect::<Vec<char>>();
    (grid, cols)
}
fn run(grid: &[char], cols: usize, initial_pos: (i64, i64), initial_dir: (i64, i64)) -> usize {
    let mut energized = HashSet::new();
    let mut pos_dir = vec![(initial_pos, initial_dir)];
    energized.insert(pos_dir[0].0);
    let mut visited = HashSet::new();
    while pos_dir.len() != 0 {
        let cur = pos_dir.pop().unwrap();
        visited.insert(cur);
        if cur.0.0 as usize >= grid.len() / cols || cur.0.1 as usize >= cols {
            continue;
        }
        energized.insert(cur.0);
        let dirs = bounce(cur.1, grid[cur.0.0 as usize * cols + cur.0.1 as usize]);
        for dir in dirs {
            let new_pos =
             (cur.0.0 + dir.0, cur.0.1 + dir.1);
            if visited.contains(&(new_pos, dir)) {
                continue;
            }
            if cur.0.0 as usize > grid.len() / cols || cur.0.1 as usize >= cols {
                continue;
            }

            if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 as usize >= grid.len() / cols || new_pos.1 as usize >= cols {
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

fn run_parts(grid: &[char], cols: usize) {
    let min_y = 0;
    let max_y = (grid.len() / cols) as i64;
    let min_x: i64 = 0;
    let max_x = cols as i64;
    let mut ans = 0;
    println!("Part 1: {}", run(&grid, cols, (0, 0), (0, 1)));
    for x in min_x..=max_x {
        ans = cmp::max(ans, run(&grid, cols, (min_y, x), (1, 0)));
        ans = cmp::max(ans, run(&grid, cols, (max_y, x), (-1, 0)));
    }
    for y in min_y..=max_y {
        ans = cmp::max(ans, run(&grid, cols, (y, min_x), (0, 1)));
        ans = cmp::max(ans, run(&grid, cols, (y, max_x), (0, -1)));
    }
    println!("Part 2: {ans}");
}
fn main() {
    let (grid, cols) = parse_grid(TEST);
    run_parts(&grid, cols);
    let (grid, cols) = parse_grid(INPUT);
    run_parts(&grid, cols);


    //run(&grid, (0, 0), (0, 1));
}
