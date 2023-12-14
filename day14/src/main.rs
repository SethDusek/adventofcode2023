const TEST: &'static str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
const INPUT: &'static str = "O#....#.........#.##...O...#.O#..#...O..O#.##.OOO.O...#..#...#OO...OOO#O##..O.O.O..#....O..#...#....
...O...O...#OO......#.........O.##....O.#O#..OO.#.O##.O.#.#.O...#O.OOOO.O#..O.#OO....O.....O.O...O..
.....#..O....O....O....O..#OO.O..#O.......O......O.O.....OO....OO......##.O.#.#....#..#.#..OOO.#.O#.
.....#..##OO..O.....#..#...O..#..O..#O..#OOO..#......O...##......OO#....#......#O.....O#.#O#O...#...
#..O..OO##O...O#O#.OOO...#.....#.#O.O#....#..........O...O#O..........#........O.O.O#O...#..OO#.....
#O.#.....##.#.OO.#..#.OO#..#.##...#..O.O..........#O.#..OO.#..O.......O.#O..O..O.O..O.....O..OOO...#
#......O.##.##...OO..O.##.##.O.#..........#..O.###O...O...##.#....###OO...#...#............O.O..#.#O
....O...O.###.#.#...O..O..O.OO...O...#........#O.....O...O#.O.O.....O#O........##.O..#OO.OO#.O##....
.....O..O#..OO.....#.O..O.#.#............#.###...#.O.###......#O#.OO.#O..OO...O.....O#O......O......
.....O.O...O#.......O.......#.OO.O##..#...O.#....#..O.....O...O.O..O.O.#.O#O...#.#..O...#O..O..OO...
O..#..O......O.....O....##OO.O..O.....#.O.#..#.#O.#.#O..............##..#..O......O...O.##..O....#..
.#...#......#..O...###....#.O#...#..O..#O......##O.OO.O.......O#OO...O#.O.#....O.O#..#.#OO.O#O.OO...
...#...O#..#..##...OO####O#.#.##....###OO#.O..........O....#O...#.....OO#.#....O....O#.##...O.#.#...
...O.OO.OO#O#.##O..O.....O...#....#O.....#.#...#O#..#..OO.O.#.#O.#.O.#..OO....O..#..#.O.#O......OO..
......#...O#...O#...O...O#..........O.....#OOO#.......#.O.O...O...#.#...#O##..O##.OO..O.......O#O...
......#...O.O...O.O.....#....#.#.....#.#...#.O#....#OO.#...OO.....#.......O.#.....O......O..##......
#O.O.O.O.OO...O###.........O.#.....O#.......O#...#O......O..#.O...O.....O...#.##O.#..##O..O#...O.#.O
O.O...#...OO#.#....OO.O...###..OO#O...O...#...##..O#.#.##..OO..O#..O...O.#...##.......#...OO.#OO.O..
..##OO#.............#...O..OO.#...##....O.O.....O.#O...O.O...O#.###....#....#....#.O..O...#....O#O#.
...O..#.......O.....O......O..O.#O#O.#......OOO..#...#..O....#..O#.#.O..O..#......OO..O..#.OO...O...
O#..........#OOOO.##..O.O#O.#.#...OO......##..#OO#.O.O#OOO.OO#.......#O.#...O#O.O..OO.....O...#O....
..#O#..O....#........#OO###...#.##.##.O....#..#..O....#O..#......O....#..O..#.....O.#......O##..#.#.
##.#..##.O.O.#.O.....O....O#.#...O..#..#....##O.#....O..OO.O.....#O.#O....O.......#.OO..O.OOO#OO#...
#.........#.#.#..OO.O.....O...#......O#O.O....#..O.#OO....O..O...#O...####O...O.....#..#....O.#.##..
#........O.O..OO.....O.....#O.O.....O....#....#...O.#.#.#.O...O#..#....#OO##.#..##O.OO...#O#.OO.#.O.
O........O.....O...O..#..#.O..............#..##.#.....O#O#.#...O#.O...O..O...#.#..O..O.OO.O....#....
..O.O.........##.#O..O...O.O....O#.#..O..##.........#.#O.O.....#.#......OO#..OO.#.O.OO...O.OO..#.O.O
#OO...O##..O#..OO.O#O.#.#.O..#..O....O.O#.#O.#........OO.O..........#....O.O....#.OO.#O..........#O.
.O..O##.#.O.##.O..O......#...###...#..#OOO.#...#O.......O.....O.#.....O.OO#...O#O.O...#OO.....O.#.#.
....#.#.O...O#OO..#..OOOO...O.............O..O..O#..O.#.O.....#.O......#OO.O#OOO........#..#..OO...#
.O...#....O..O##.OO.###.O..OO.O....#.O..##.OO#O.....##..........###.O...O#..OO..O.O#O.....O.#.......
#.O......OOO......O.#..O..##...#O.O#.O#..OO..#..#O.OOO#..#.O.O...O..O....#O...O...#..#.##.O...#.O#.#
...O...O....O.#.#OOO.....O#O......O#..OO........O.....O..O.......##O..#...OOOOO#....O..#.O......#..O
#.....#....O.....O..O.O.#O#O.O.#.....#..#O.....#.#.#...OO.OOO#O##.##O.O.###.O..O...O#.#.O......##..O
.O.#..#.O##..#..OO#.#.O#.#.O..O.....#....O##...O.....##O..##...O...O...O.#.#...#O..OO...O....#....#.
O#..#.###....O.#..#.O.O...O.O....#.......#.O.#O.#...#.O.O#..#.#..O..O.O#...O....O.O.O##...#.....OOO.
...#.#O.#......OO#.##..O..#.#.O.###O...O.#O.O..O..#.....O..#O.O.O....OO##O..OO...O##.#....O.O...O...
.OO.O#..#.OO.O.O..O#OOO.OO.##.O........O..#.O..#..O.............O.O..#.OOO.#................##O.....
....##O...O..O..O.........#..O..O...O...O..O.#.....O......O#.O#.#.......OO...O.#.O#.....#.#........#
...#.O...OO.OOO.#O............O..O#..#...#..OO.....O........O..#.O..O...O#O....##O.O...O.O..#.O....#
...#.#O..O#.#..O....O..OO....O..O#O..........OO.O..O.......#.#..#....O..##.O.#....O#O.....#OO.....O.
..#.O...#.O..O#.....O...OO##.#.OO.O.O.O.O.#.O#...#O...O..###.....O#..#...........O.##O......#..O.#..
O.....O......O.O....#.O.O..#.#...O...#.O.O#OO..#...O#......O.OO.O.O..#...#.#.....O#....OOO..OOO.O..#
O....#O#....#..O#....##........O##O......O##.#O.....O.O.....#.##..#.OO..##....#.O#...OO........#..O.
.#....O...O.#......#O##.#......#..#.#...............OO.OOOOO.O.OO..OO.O.#.O.......#O....O#.#O..OO.O#
#O.OOO#OO....##.O.#.O#.O...#....#.....#....OO#.O.#OOOO....#..#.#.##.O...#...OOO...#......O......#.#.
O.#.#O.#.#.#.O.#.O........#.O.#.OO#O.O##.#..#...O.O.....O.#...O#.....#.OO..O#.OO.#...#.O....#.......
..O..O.O.O..###....#O...#O#.##O......#O....OO.O..O.#...O#.O....#..#.#.O.........#OO..........O#..O#.
#.#...........#.......#..OO..#.....##...#........#O.#O..#O....#OO.O.O....#.O..O#.....##.O###O##....O
..OO..OOOO.....O...#.O.#....O.....O.##O#...#..O...#...O.OOO.O..#.#...#....O#.#...#.O.O.O.....##..O.#
..OOO..#.#O..OO.O......O...#.#..O.....O.OO.O.O.OO...O..#.O.O.....#..O..#.##.O....#.#O.OOOOO#..O#.O..
.....#.....O......#..#.....O..........O####..O..O.#O#.....#O...#..O.#...#....#.OOOO.OO.O.O.O#...O..#
O.#.#O.#.O...O.OO..O......#...#.O#......O...##O#..O.OO.#....O#...OO...#...................O..#O...O.
.#.OO...OO....O#......O.O#O#O.O.O....O...O.OO#.#O.O.O.#O.O...#.....O..##.#..##..O#...O#O#...O...#...
.#.....#..##O.OOO#.O#.##.O..#..O#O..#..#...O#O.O##....O...O.O..O#..#.O.O.O.......#O..O..#O.O...O.#.O
......O..O.#..#.##.O.O....OO..O.....#.O..#O...#O.O..#..#..O...O..O..#.#O......O..O......O.....O.##..
..O....#O..O.#O...#O...#.O.#...O...#.#.#...O..O.#.......#...O...OO.#O..#OO..OO.O#OOO###.......#.O..O
..O..O...#..#....#....O..O#.#.O.........O.#.##.#O..#.OO.#O.O.O#.....#O..O....O#.........#.#.#.....O#
......O.#O#..#OO.#...O..#....#O.....#.......##.#.............O.#OO..#.O...#..O#.####.#OO..O.O...#.#.
#......O....O..O....O.....#.O......#.........#.O.O..#...#.O..O#...#.O#.#O.#..#.O....O.#...#O..O...#.
.O...#...OO.#O.....#...O#...O.O#.#.O.#O...OO..#.O##O#.#OO#O...O#......##.O.##O.#......O.........OO..
..O#.O..#OO..O........O.OO.O..O...O.#....#..#.#O#..O...#.....#.#OO.O.......#..O...OO..O.OO.#.#O.OO..
OO.O........O.#..O...O.O...OO#.O#O.O.O...OOO.O.#...O.......OO.#.#..O#.#..#..O...OO.#..O#O.#..#....O.
O..#..O.O#.OO..#...O#........#...#.O...O...#...#.#.O...#...#O.OO.#......O...O.O........#.##O#.O..#O.
OO.OO.#..#..#....O..O.O..O..........OO#.....##OO.O..##.......#.....O.......#...O....O..#........#O.O
O#.....O.O...#..#..O#..#.O.O##.O.#O...#O..O##O....O.O##.#...###...........O#.O...........O..O.O.....
....O......##....O.........#O...O#.#.#....O.#..........#.#.O..O..#..O...O....OOO......OO.O..O#..OO.O
.O.#..O........O.O.......O..##.#..OOO..#.#.....##.O.O.#.#...#.O.#.###.O.....O....O.....O..O.O.O#...#
...O..O.O#.....O.O.......O.O.#..#O#.O#..OOOO..#......O.O.#....O...#.#..#O....#..OOO.O.OO.#O.O....O#.
...O.....#....#OOO#.O#...#.O......#.#O.O...OO.O#.OO.OO..O...#........O#.#.O#.....O..O#.OOO........#.
O..O..O...O.............O..O.O.OO...O..O.....O...O#..##OO#O..#.##...#O..#O#......O##.O........#..#..
.OO.#.#O...O...O.#.O.O.O.........O..O..O..OOO....#..#.O#..O......#..#O..#....#..#.......OO.......O#.
#.....#.O#.....#..O..O.O..O...O.##O.O..#..O.......O.O..O........#....O.#...#OO.O..O...O..O.#..O..O.O
#..##.#O##.##..#.......##.O..OO.#.#O#O#...#.#.O....OOO.##OOO.OO.O#.O#........#...#...#.##.OOO..#..#.
....O.##O#O..#O..O.O#....OO....O...#.O..O.O..#O..O..#..OOO.....##OO.....#.O.#OO...#.O.OOO.O....#....
.O........#O...O.O.O.O#..O#.O.....#OO.#.O..#.#.O.....O....O......O....O..O.#..##.....O.#.....O..O...
O...O......#..O#.......#O.....O#.#.....#.O.OO.O.#O..O##.....OO#..O.O.O........#O..O.....#..........O
..#O.O.#.O.#..#......O#..O#..O..#...O..#.O.O#.##..#.#O..O..#.#...##.#...#..O#......#...O#O.O.#..#...
.OO...O..O...O.O..O..O#.....O.#O#.O#O.#.....O.#...#..#...O..OO.....O..O.O....#...#O#.#.OO#....O....O
.#...O.........#.O.O#.OO.#O..O....O.....O....##.....O..#OOO.....#...#.#OO..#O.O.O...#O.OO.O.#.......
..OO.#O......O.O..##.#........OO..O#..O...O..#..#...#O..##O...O#...##.#..O..#......#..O.O#.O.#...O.O
..##.O..###..OO.O#O..#.......O#...#.O#..OO.##O.O.O........#O#.#.OO....#.##.O.OO#.....O.O##........#.
..O.OO.O##.O...##O...#O..#O.#.O...#O.##..#.O.#..##....##.O....O....O.....###.....#O..#..O..#O...O...
.##.....#.....O#.O...#.....#.##.OO..OOO...OO#..O......#........#O.O..O.O...#...OO....O.#...O.O...O..
...O.....O.#....O...#..O.O......#..#OO.#.O.O#.#O....#.##...O...##O.O.O#.........O.#O......#O.......O
..###O..#O.#.O.....O.#........O.#........#O##.#..O#.##.O.O........O###.#.O....O.....###....O##O.....
#O#.....#.....O..............O...#..O..#O.O...........###.O.#.....##O.....O#O..#.....O.OO....O#O....
.#...O...O..#.#...............O.O#....O..#.#OO.O..O........O....O..O......##..#.#OO#...O#..O##......
.OO.O...O##O.#O#..O.......O..##.#O#.....#..O..O#.#O#.#...#...#..O..##..#....O.....#.#..O....O.##.O#.
#...O#.#O.O...#.O....O#.O#..O..O......OO#.O....#.....OO#O....#O.OOO...#OO..#..#O...#.#O..OO.#...O...
.O...O..O.O....#....#O...#.....#...O...#..OOO.....OO#OOO.O..#O.#....#..O...#...O..O.O##...#O.O.OO..#
O#O..O#......#.#O..#O....OO#O#...#...O....##...#.O...O.#O...OO....O.#.O.O...O#........#O.O..O#..#..O
OOO#....O...##.O.........OO..O.OOOOO.O..O..#..###.O.#....##.....O.O.O#.#..OOO....OO.....OO....O...#.
#O###O...O...#..OO...#O.....#.....O.O...O.#..OO#..##....O..##..O##.......O..O.O#OOO...O..#.....##...
..#O.....#.O#O#.O.#.#....#O..#O.O.O..#..O#..O.O.O...OO..#.#..##..O.O...OO..O.##.O.OO.#.#..##.......#
O...OO.O......#..#...O.#.O.#O...#.O.OO#...O....#OOO..#..O..O.....O.....#.O.#O#...OO.O#.OO..O.##...#.
.....#OO....O.O#O......O.O.O.#.O..O#O#...#O...#.#O...#.O..#.OO.O....OO...O#........#O...#.#........O
.O....#..OOO........#.O.#O.#.........#.#....O#OO#.......O..O#O.###.O....O.O.O......O.#.O..O..#...O.O
#.O.#O..##.OO...O##O......O#.OO........OO...O#....#....#..O#........O#..O.O...O.#O...#...#.#.....O##
.O.O.O.OO..OO.#..#....O....O.O#...O...OOO..#.OO#.....##........O...O#O.#..OO##.O.#..#..#O#.O..OO##..";

fn run(input: &'static str) {
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut cloned = grid.clone();
    roll(&mut cloned, (-1, 0));

    let total = cloned
        .iter()
        .enumerate()
        .map(|(r, l)| {
            l.iter()
                .filter(|&&c| c == 'O')
                .map(|_| grid.len() - r)
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{total}");
}

fn roll(grid: &mut [Vec<char>], dir: (isize, isize)) {
    let mut stable = false;
    while !stable {
        stable = true;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let pos = (i as isize + dir.0, j as isize + dir.1);
                if pos.0 < 0 || pos.1 < 0 || pos.0 >= grid.len() as isize || pos.1 >= grid[0].len() as isize {
                    continue;
                }
                let pos = (pos.0 as usize, pos.1 as usize);
                if grid[i][j] == 'O' && grid[pos.0][pos.1] == '.' {
                    grid[i][j] = '.';
                    grid[pos.0][pos.1] = 'O';
                    stable = false;
                }
            }
        }
    }
}
fn main() {
    run(TEST);
    run(INPUT);
}
