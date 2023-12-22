use std::{collections::HashMap, collections::HashSet, ops::RangeInclusive, str::FromStr};
const TEST: &'static str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";
const INPUT: &'static str = include_str!("../input.txt");
type Grid = HashMap<(i64, i64), u8>;
fn parse(input: &'static str) -> Vec<((i64, i64), (i64, i64), (i64, i64))> {
    input
        .lines()
        .map(|l| l.split_once("~").unwrap())
        .map(|(start, end)| {
            (
                start.split(",").map(i64::from_str).map(Result::unwrap),
                end.split(",").map(i64::from_str).map(Result::unwrap),
            )
        })
        .map(|(start, end)| start.zip(end))
        .map(|mut iter| {
            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .collect()
}

fn run1(input: &mut Vec<((i64, i64), (i64, i64), (i64, i64))>) {
    input.sort_by_key(|(_, _, (start, _))| *start);
    let max_x = input.iter().map(|((_, end), _, _)| *end).max().unwrap() + 1;
    let max_y = input
        .iter()
        .map(|((_, _), (_, end), _)| *end)
        .max()
        .unwrap()
        + 1;
    let mut grid = vec![vec![1; max_x as usize]; max_y as usize];
    let mut brick_at: Vec<Vec<usize>> = vec![vec![usize::MAX; max_x as usize]; max_y as usize];
    let mut disintegrable: HashSet<usize> = (0..input.len()).collect();
    let mut supported_by: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut supports: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, ((start_x, end_x), (start_y, end_y), (start_z, end_z))) in
        input.iter().copied().enumerate()
    {
        let mut highest = 1;
        for y in start_y..=end_y {
            for x in start_x..=end_x {
                highest = std::cmp::max(highest, grid[y as usize][x as usize]);
            }
        }
        // number of bricks at (start_x..=end_x) (start_y..=end_y)
        let mut unique_bricks = HashSet::new();
        for y in start_y..=end_y {
            for x in start_x..=end_x {
                if grid[y as usize][x as usize] == highest
                    && brick_at[y as usize][x as usize] != usize::MAX
                {
                    unique_bricks.insert(brick_at[y as usize][x as usize]);
                }
                grid[y as usize][x as usize] = highest + (start_z..=end_z).count();

                brick_at[y as usize][x as usize] = i;
            }
        }
        supported_by.insert(i, unique_bricks.iter().copied().collect());
        unique_bricks.iter().copied().for_each(|supporting| {
            supports
                .entry(supporting)
                .and_modify(|v| v.push(i))
                .or_insert(vec![i]);
        });
        if unique_bricks.len() == 1 {
            disintegrable.remove(unique_bricks.iter().next().unwrap());
        }
    }
    println!("Part 1: {}", disintegrable.len());

    let mut sum = 0;
    for i in 0..input.len() {
        let mut count = 0;
        let mut stack = Vec::new();
        stack.push(i);
        let mut children = HashSet::new();
        while stack.len() != 0 {
            let cur = stack.pop().unwrap();
            children.insert(cur);
            count += 1;
            if !supports.contains_key(&cur) {
                continue;
            }
            for &supporting in &supports[&cur] {
                if supported_by[&supporting]
                    .iter()
                    .any(|s| !children.contains(s))
                {
                    continue;
                }
                stack.push(supporting);
            }
        }
        sum += count - 1;
    }
    println!("Part 2: {sum}");
}

fn main() {
    let mut parsed = parse(TEST);
    run1(&mut parsed);
    let mut parsed = parse(INPUT);
    run1(&mut parsed);
}
