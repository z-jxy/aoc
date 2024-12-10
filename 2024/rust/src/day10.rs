use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashSet;

const DIRECTIONS: [(i32, i32); 4] = [
    (0, 1),  // right
    (1, 0),  // down
    (0, -1), // left
    (-1, 0), // up
];

#[derive(Debug)]
struct TrailNode {
    r: usize,
    c: usize,
    value: u32,
}

fn search(
    grid: &[Vec<u32>],
    node: &TrailNode,
    depth: usize,
    score: &mut usize,
    nines: &mut HashSet<(usize, usize)>,
    p2: bool,
) -> bool {
    if depth == 9 {
        match p2 {
            true => *score += 1,
            false => {
                if !nines.contains(&(node.r, node.c)) {
                    *score += 1;
                    nines.insert((node.r, node.c));
                }
            }
        }
        return true;
    }
    let nr = grid.len();
    let nc = grid[0].len();

    let r = node.r;
    let c = node.c;

    for direction in DIRECTIONS.iter() {
        let (dx, dy) = direction;
        let nx = r as i32 + dx;
        let ny = c as i32 + dy;

        if nx >= 0 && nx < nr as i32 && ny >= 0 && ny < nc as i32 {
            let nx = nx as usize;
            let ny = ny as usize;

            if grid[nx][ny] == node.value + 1 {
                let nx_val = grid[nx][ny];
                search(
                    grid,
                    &TrailNode {
                        r: nx,
                        c: ny,
                        value: nx_val,
                    },
                    depth + 1,
                    score,
                    nines,
                    p2,
                );
            }
        }
    }

    false
}

#[aoc_generator(day10)]
fn parse(input: &str) -> (Vec<TrailNode>, Vec<Vec<u32>>) {
    let mut trailheads = vec![];
    let grid = input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, ch)| {
                    let digit = ch.to_digit(10).unwrap();
                    if digit == 0 {
                        trailheads.push(TrailNode { r, c, value: 0 });
                    }
                    digit
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (trailheads, grid)
}

#[aoc(day10, part1)]
fn part1((trailheads, grid): &(Vec<TrailNode>, Vec<Vec<u32>>)) -> usize {
    let mut score = 0;
    for node in trailheads.iter() {
        search(&grid, &node, 0, &mut score, &mut HashSet::new(), false);
    }

    score
}

#[aoc(day10, part2)]
fn part2((trailheads, grid): &(Vec<TrailNode>, Vec<Vec<u32>>)) -> usize {
    let mut score = 0;
    for node in trailheads.iter() {
        search(&grid, &node, 0, &mut score, &mut HashSet::new(), true);
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(&EXAMPLE)), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(&EXAMPLE)), 81);
    }
}
