use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &'static str = ".#..........#.....................#........................................................#....................................#...........
...................#........................................#......................#........................................................
...........................................#......#..................#...................................................#..................
....#......................#...............................................#.........................#.....#......#................#........
.................................................................#.......................#..................................................
..............................................................................................................................#.............
.......#........#.............#...................................................#...................................#.....................
.........................#...................................#............................................................................#.
....................#.............#.........#..........................................#...............#....................................
.....................................................#...............#.............................................................#........
...........................................................................................................................#................
..........................................................#.........................................#...............#.......................
..............................................................................................#.............................................
.#........#........................#......#..............................................#..............................#...............#...
...............#......#...............................#.......#.........#......#.............................#.................#............
...............................#..............#.............................................................................................
............................................................................................................................................
.........................#.......................................#..........................................................................
..................#........................#......................................#........................................#................
..................................................#.......#..............................................#........................#.......#.
...#........#........................#....................................#...............#.................................................
............................................................................................................................................
.......#........................#....................................................................................#......................
...............#...........#........................................................................#.......................................
.............................................#................#.........#.............................................................#.....
...........#...........................#................#...........................#.......................................................
..................................................#.......................................#.............#...................................
.....#..................#..........#..............................#.............................#........................................#..
..............................#...............................................................................................#.............
...................................................................................................................#........................
..............#..........................................#.............#...........................#..............................#.........
...#...................................#.........................................#..........................................................
..................#...................................................................#................................................#....
..........................#...................................#.........................................#...................................
............................................................................................................................................
.........#...........#..........#..........................................#....................#..............#........#.....#...........#.
....................................................#.......................................................................................
.............................................#..............................................................................................
......................................#..................#..............................#...........#.......#...............................
............................#..................................#..............#.............................................................
...............#............................................................................................................................
...........................................#............................#.......................#...........................................
.....#..............#.................................................................................#...........................#.........
............................................................................................................................................
........................................#..........#.................#..........................................#...........................
.............................................#.................................#...........#................................................
...............................#.......................#...........................................#.......#....................#...........
#........#.............#...............................................................................................#...................#
.....................................#........................#..........#.........#........................................................
.................#............................................................................#.............................................
..........................#.......................................#.................................................#.......................
.....................................................#...................................#....................................#.............
..#.........................................................................................................................................
.............#.....#.......................#.............#....................................................#........................#....
......#........................................................................................#............................................
.................................#.................................#........................................................................
.................................................................................#.......................#................................#.
#.....................................................#...................................................................#.....#...........
.........#..................................................#......................................#........................................
..............................#.............#.................................................................#.......#.....................
........................#............................................................#......#................................#..............
.............#..............................................................................................................................
.....#........................................................#............#..........................#...........................#.........
...........................#......#.................................................................................#...................#...
.........#.....................................#...............................#............................................................
..................#.......................................#.......#................................#........................................
...............................#.....................#...................................#................................#.................
..............#..........................#.............................#............#.....................#.....................#.........#.
.................................................................................................................#..........................
......#.....................................................................................................................................
...........#.............#.........#........................................................................................................
...................................................................#..............#..................#......................................
..................#...........................................................................................#.............................
........#.................................#..............#.................................................................#..........#.....
...#.....................................................................#..............#.......#...........................................
............................................................................................................................................
............................................................................................................................................
.................................#................#..........................................................#...............#.............#
..............#............................................................#................................................................
......#.....................#..........................#........#.....#............#..............#.........................................
....................#..................................................................................#..................#.................
....................................#.........#........................................................................................#....
..........................................................#..............#..................................................................
................#...............................................................................................#.............#.............
............................................................................................................................................
............................#...................#......#.............#..............................................#.......................
.............#...................................................................................#........................................#.
......................#..........................................................#.......#..................................................
#...........................................................#...............................................................................
.........#.........................#.................#....................................................#.................#...............
............................................................................................................................................
...................#.............................................#.....#....................................................................
........................#.........................#.....................................................................................#...
.....#........................................................................................................................#.............
.......................................#...............................................................#.........#..........................
............................#...........................#...........#......................#..............................#.......#.........
..#................................#...........#............................................................................................
.................#..................................#...................#...................................................................
..........#.................................................#................................................#..............................
.....................#...............................................................................#......................................
..............#..................#..............................#.............#....................................#.........#........#.....
...#.............................................................................................#..........................................
............................................#..........................................#....................................................
.........#...............................................#..............................................................#...................
................#.........#.................................................................................................................
.....#..................................#............................#..............................#........#..................#..........#
...............................#............................................................................................................
....................................................#........#..............................................................................
........#..........................#.........#...............................................................................#..............
....................................................................................................................#.......................
....................#........#......................................#.........#.....#.....#.................................................
................................................................................................................................#.....#.....
...............#..........................................#...............#...............................#.................................
...#..............................#.........................................................................................................
........................................................................................#............#....................................#.
............................................................................................................................#...............
...................................................................................#............................#...........................
#........#........#........#..........#.........#...........................................................................................
.............................................................#..............................#............................#..................
................................#.......................#...................................................................................
.......................#................................................#...............#.........#..................#................#.....
............#..................................................................#........................#...................................
............................................................................................................................................
......#.....................#.......#......#..............#......#..........................................................................
.....................#......................................................................................................................
................................#................................................#.............#...........................#.............#..
..............................................................................................................#.............................
............#............#..........................................................................#.............................#.........
.....#...........#.....................#.............................#.....#........................................#.......................
................................................#...........#..........................#....................................................
............................................................................................................................................
#..........................#.....#................................................................#......................................#..
..........................................#.................................................................#.............#.................
................#.........................................................#.................................................................
.......#..............................#..............................................#..........................................#...........
........................#...................................#...............................................................................
............................................................................................................................#...............
..............................#.....................#.............................#...........#.................#...........................
...............................................#................#.....#................................................#....................
...................#...................#.................#.................#.........................#......................................
";
const TEST: &'static str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

fn run(input: &'static str, dist: usize) {
    let empty_rows = input
        .lines()
        .map(|l| {
            if l.chars().all(|c| c == '.') {
                1i64
            } else {
                0i64
            }
        })
        .scan(0, |sum, val| {
            *sum += val;
            Some(*sum)
        })
        .collect::<Vec<i64>>();
    //println!("{:?}", empty_rows);
    let mut empty_cols = Vec::new();
    let lines = input.lines().collect::<Vec<&str>>();
    for i in 0..lines[0].len() {
        let mut all = true;
        for r in 0..lines.len() {
            if lines[r].bytes().nth(i).unwrap() != b'.' {
                all = false;
            }
        }
        let val = if all { 1 } else { 0 };
        if empty_cols.len() != 0 {
            empty_cols.push(empty_cols[empty_cols.len() - 1] + val);
        } else {
            empty_cols.push(val);
        }
    }
    //println!("{:?} {:?}", empty_rows, empty_cols);
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((y as i64, x as i64), c as char))
                .filter(|&(_, c)| c == '#')
                .map(|(xy, _)| xy)
        }).collect::<Vec<(i64, i64)>>();

    let mut total = 0;
    for i in 0..galaxies.len() - 1 {
        let cur = galaxies[i];
        for &other in &galaxies[i + 1..] {
            let (min_y, min_x) = (cmp::min(cur.0, other.0), cmp::min(cur.1, other.1));
            let (max_y, max_x) = (cmp::max(cur.0, other.0), cmp::max(cur.1, other.1));
            let distance = max_x - min_x + max_y - min_y;
            let empty_space = empty_rows[max_y as usize] - empty_rows[min_y as usize] + empty_cols[max_x as usize] - empty_cols[min_x as usize];
            total+=distance + empty_space * dist as i64;
        }
    }
    println!("{}", total);
}
fn main() {
    run(TEST, 1);
    run(INPUT, 1);
    run(TEST, 999_999);
    run(INPUT, 999_999);
}
