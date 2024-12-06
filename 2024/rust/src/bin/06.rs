use std::collections::{HashSet, VecDeque};

fn solve(grid: &[Vec<char>]) -> (usize, usize) {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = vec![vec![false; cols]; rows];

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find_map(|(c, &cell)| if cell == '^' { Some((r, c)) } else { None })
        })
        .unwrap();

    let mut queue = VecDeque::from([start]);

    let mut direction = (-1i32, 0i32);

    while let Some((r, c)) = queue.pop_front() {
        visited[r][c] = true;

        let nr = r as i32 + direction.0;
        let nc = c as i32 + direction.1;

        if nr < 0 || nr as usize >= rows || nc < 0 || nc as usize >= cols {
            break;
        }

        if grid[nr as usize][nc as usize] == '#' {
            match direction {
                (-1, 0) => direction = (0, 1),
                (0, 1) => direction = (1, 0),
                (1, 0) => direction = (0, -1),
                (0, -1) => direction = (-1, 0),
                _ => unreachable!(),
            }
            queue.push_back((r, c));
            continue;
        }

        queue.push_back((nr as usize, nc as usize));
    }

    let p1 = visited.iter().flatten().filter(|&&v| v).count();

    // don't include the starting position
    visited[start.0][start.1] = false;

    let mut potential_positions = vec![];
    for i in 0..visited.len() {
        for j in 0..visited[i].len() {
            if visited[i][j] {
                potential_positions.push((i, j));
            }
        }
    }

    let mut count = 0;
    for (_r, _c) in potential_positions.iter() {
        let mut looped = false;
        let mut queue = VecDeque::from([start]);

        let mut direction = (-1i32, 0i32);

        let mut visited = HashSet::new();
        let mut grid = grid.to_vec();

        grid[*_r][*_c] = '#';

        while let Some((r, c)) = queue.pop_front() {
            visited.insert((r, c, direction));

            let nr = r as i32 + direction.0;
            let nc = c as i32 + direction.1;

            if nr < 0 || nr as usize >= rows || nc < 0 || nc as usize >= cols {
                break;
            }

            if grid[nr as usize][nc as usize] == '#' {
                // rotate 90 degrees to the right and continute
                match direction {
                    (-1, 0) => direction = (0, 1),
                    (0, 1) => direction = (1, 0),
                    (1, 0) => direction = (0, -1),
                    (0, -1) => direction = (-1, 0),
                    _ => unreachable!(),
                }
                queue.push_back((r, c));
                continue;
            }

            if visited.contains(&(nr as usize, nc as usize, direction)) {
                looped = true;
                break;
            }

            queue.push_back((nr as usize, nc as usize));
        }

        if looped {
            count += 1;
        }
    }

    (p1, count)
}

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    solve(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_p1() {
        let (p1, _) = solve(
            &EXAMPLE
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );
        assert_eq!(p1, 41);
    }

    #[test]
    fn test_p2() {
        let (_, p2) = solve(
            &EXAMPLE
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );
        assert_eq!(p2, 6);
    }
}
