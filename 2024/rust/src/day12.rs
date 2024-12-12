use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

type Regions = Vec<Vec<(usize, usize)>>;

#[aoc_generator(day12)]
fn parse(input: &str) -> Regions {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut regions = Vec::new();

    let mut visisted = vec![vec![false; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            if visisted[r][c] {
                continue;
            }
            let plot_char = grid[r][c];
            visisted[r][c] = true;
            let mut plots: Vec<(usize, usize)> = vec![(r, c)];
            gather_plots(&grid, plot_char, r, c, &mut plots, &mut visisted);

            regions.push(plots);
        }
    }

    regions
}

#[aoc(day12, part1)]
fn part1(graph: &Regions) -> usize {
    graph
        .iter()
        .fold(0, |acc, edges| acc + edges.len() * perimeter(&edges))
}

#[aoc(day12, part2)]
fn part2(graph: &Regions) -> usize {
    graph
        .iter()
        .fold(0, |acc, edges| acc + edges.len() * sides(&edges))
}

fn gather_plots(
    grid: &[Vec<char>],
    plot_char: char,
    r: usize,
    c: usize,
    plots: &mut Vec<(usize, usize)>,
    visited: &mut [Vec<bool>],
) {
    for (dr, dc) in DIRECTIONS.iter() {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[r].len() as i32 {
            continue;
        }

        if visited[nr as usize][nc as usize] {
            continue;
        }

        if grid[nr as usize][nc as usize] == plot_char {
            plots.push((nr as usize, nc as usize));
            visited[nr as usize][nc as usize] = true;

            gather_plots(grid, plot_char, nr as usize, nc as usize, plots, visited);
        }
    }
}

fn perimeter(plots: &[(usize, usize)]) -> usize {
    let mut sum = 0;

    for &(r, c) in plots.iter() {
        let mut sides = 4;
        for (dr, dc) in DIRECTIONS.iter() {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if plots
                .iter()
                .any(|&(r, c)| r == nr as usize && c == nc as usize)
            {
                sides -= 1;
            }
        }

        sum += sides;
    }

    sum
}

fn sides(region: &[(usize, usize)]) -> usize {
    let mut side_count = 0;
    for &(dx, dy) in DIRECTIONS.iter() {
        let mut sides = HashSet::new();
        for &(x, y) in region {
            let tmp_x = x as i32 + dx;
            let tmp_y = y as i32 + dy;

            let tmp = (tmp_x as usize, tmp_y as usize);
            if !region.contains(&tmp) {
                sides.insert(tmp);
            }
        }

        let mut remove = HashSet::new();
        for side in &sides {
            let mut tmp_x = side.0 as i32 + dy;
            let mut tmp_y = side.1 as i32 + dx;

            while sides.contains(&(tmp_x as usize, tmp_y as usize)) {
                remove.insert((tmp_x as usize, tmp_y as usize));
                tmp_x += dy;
                tmp_y += dx;
            }
        }
        side_count += sides.len() - remove.len();
    }

    side_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"AAAA
BBCD
BBCC
EEEC"#;

    const EXAMPLE_2: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE_1)), 140);
        assert_eq!(part1(&parse(EXAMPLE_2)), 1930);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE_1)), 80);
    }
}
