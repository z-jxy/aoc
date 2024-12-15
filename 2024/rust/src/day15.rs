use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

const DIRECTIONS: [(i32, i32); 4] = [
    (-1, 0), // up
    (1, 0),  // down
    (0, -1), // left
    (0, 1),  // right
];
const UP: (i32, i32) = DIRECTIONS[0];
const DOWN: (i32, i32) = DIRECTIONS[1];
const LEFT: (i32, i32) = DIRECTIONS[2];
const RIGHT: (i32, i32) = DIRECTIONS[3];

type ParsedInput = (Vec<Vec<u8>>, Vec<(i32, i32)>, (usize, usize));

#[aoc_generator(day15, part1)]
fn parse(input: &str) -> ParsedInput {
    let (top, bottom) = input.split_once("\n\n").unwrap();

    let grid = top
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let instructions = bottom
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '^' => UP,
                'v' => DOWN,
                '<' => LEFT,
                '>' => RIGHT,
                _ => unreachable!(),
            })
        })
        .collect::<Vec<_>>();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find_map(|(c, &cell)| if cell == b'@' { Some((r, c)) } else { None })
        })
        .unwrap();

    (grid, instructions, start)
}

fn solve((grid, moves, start): &ParsedInput) -> usize {
    let mut grid = grid.clone();
    let mut pos = *start;

    for (dr, dc) in moves {
        if let Some(new_pos) = r#move((*dr, *dc), &mut grid, (pos.0, pos.1)) {
            pos = new_pos;
        }
    }

    let mut total = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == b'[' || grid[r][c] == b'O' {
                total += 100 * r + c;
            }
        }
    }

    total
}

#[aoc_generator(day15, part2)]
fn parse2(input: &str) -> ParsedInput {
    let (top, bottom) = input.split_once("\n\n").unwrap();

    let grid = top
        .lines()
        .map(|line| {
            line.bytes()
                .flat_map(|c| match c {
                    b'#' => b"##",
                    b'O' => b"[]",
                    b'.' => b"..",
                    b'@' => b"@.",
                    _ => unreachable!(),
                })
                .copied()
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    let instructions = bottom
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '^' => UP,
                'v' => DOWN,
                '<' => LEFT,
                '>' => RIGHT,
                _ => unreachable!(),
            })
        })
        .collect::<Vec<_>>();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find_map(|(c, &cell)| if cell == b'@' { Some((r, c)) } else { None })
        })
        .unwrap();

    (grid, instructions, start)
}

#[aoc(day15, part1)]
fn part1(input: &ParsedInput) -> usize {
    solve(input)
}

#[aoc(day15, part2)]
fn part2(input: &ParsedInput) -> usize {
    solve(input)
}

fn r#move(
    dir: (i32, i32),
    griddy: &mut Vec<Vec<u8>>,
    (r, c): (usize, usize),
) -> Option<(usize, usize)> {
    let (rr, cc) = ((r as i32 + dir.0) as usize, (c as i32 + dir.1) as usize);
    let mut q = VecDeque::from([(r, c)]);
    let mut seen = HashSet::new();

    while let Some((rr, cc)) = q.pop_front() {
        if !seen.insert((rr, cc)) {
            continue;
        }
        let (r2, c2) = ((rr as i32 + dir.0) as usize, (cc as i32 + dir.1) as usize);
        match griddy[r2][c2] {
            b'#' => return None,
            b'O' => {
                q.push_back((r2, c2));
            }
            b'[' => {
                q.push_back((r2, c2));
                q.push_back((r2, c2 + 1));
            }
            b']' => {
                q.push_back((r2, c2));
                q.push_back((r2, c2 - 1));
            }
            _ => continue,
        }
    }
    while !seen.is_empty() {
        for (rr, cc) in seen.iter().copied().sorted() {
            let (r2, c2) = (rr + dir.0 as usize, cc + dir.1 as usize);
            if !seen.contains(&(r2, c2)) {
                griddy[r2][c2] = griddy[rr][cc];
                griddy[rr][cc] = b'.';
                seen.remove(&(rr, cc));
            }
        }
    }

    Some((rr, cc))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;

    const EXAMPLE2: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 2028);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse(EXAMPLE2)), 10092);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2(EXAMPLE2)), 9021);
    }
}
