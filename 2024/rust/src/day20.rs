use crate::GridExt;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Cheat {
    turns_saved: usize,
}

type Parsed = (Vec<Vec<u8>>, (usize, usize), (usize, usize));

#[aoc_generator(day20)]
fn parse(input: &str) -> Parsed {
    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    let start = grid.get_unique_position(b'S').unwrap();
    let end = grid.get_unique_position(b'E').unwrap();
    (grid, start, end)
}

fn solve(
    start: (usize, usize),
    end: (usize, usize),
    grid: &[Vec<u8>],
    max_distance: usize,
) -> Vec<Cheat> {
    let mut cheats = Vec::new();

    let mut start_distances = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    let mut end_distances = vec![vec![usize::MAX; grid[0].len()]; grid.len()];

    // calculate distances from start
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((start, 0));
    while let Some((pos, dist)) = queue.pop_front() {
        if start_distances[pos.0][pos.1] != usize::MAX {
            continue;
        }

        start_distances[pos.0][pos.1] = dist;

        for (dy, dx) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = (pos.0 as isize + dy, pos.1 as isize + dx);
            if new_pos.0 < 0 || new_pos.1 < 0 {
                continue;
            }
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            if new_pos.0 >= grid.len() || new_pos.1 >= grid[0].len() {
                continue;
            }
            if grid[new_pos.0][new_pos.1] == b'#' {
                continue;
            }
            queue.push_back((new_pos, dist + 1));
        }
    }

    // calculate distances from end
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((end, 0));
    while let Some((pos, dist)) = queue.pop_front() {
        if end_distances[pos.0][pos.1] != usize::MAX {
            continue;
        }

        end_distances[pos.0][pos.1] = dist;

        for (dy, dx) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = (pos.0 as isize + dy, pos.1 as isize + dx);
            if new_pos.0 < 0 || new_pos.1 < 0 {
                continue;
            }
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            if new_pos.0 >= grid.len() || new_pos.1 >= grid[0].len() {
                continue;
            }
            if grid[new_pos.0][new_pos.1] == b'#' {
                continue;
            }
            queue.push_back((new_pos, dist + 1));
        }
    }

    let original_distance = end_distances[start.0][start.1];

    // look for cheats
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            // let mid_point = (i, j);
            if grid[i][j] == b'#' {
                continue;
            }
            if start_distances[i][j] == usize::MAX {
                continue;
            }

            // check all points within manhattan distance
            for di in -(max_distance as isize)..=max_distance as isize {
                for dj in -(max_distance as isize)..=max_distance as isize {
                    let manhattan = di.abs() + dj.abs();
                    if manhattan as usize > max_distance {
                        continue;
                    }

                    let end_i = i as isize + di;
                    let end_j = j as isize + dj;

                    if end_i < 0 || end_j < 0 {
                        continue;
                    }
                    let end_i = end_i as usize;
                    let end_j = end_j as usize;

                    if end_i >= grid.len() || end_j >= grid[0].len() {
                        continue;
                    }

                    if grid[end_i][end_j] == b'#' {
                        continue;
                    }

                    if end_distances[end_i][end_j] == usize::MAX {
                        continue;
                    }

                    let new_distance =
                        start_distances[i][j] + end_distances[end_i][end_j] + manhattan as usize;
                    let turns_saved = original_distance.saturating_sub(new_distance);

                    if turns_saved > 0 {
                        cheats.push(Cheat { turns_saved });
                    }
                }
            }
        }
    }

    cheats
}

#[aoc(day20, part1)]
fn part1((input, start, end): &Parsed) -> usize {
    solve(*start, *end, input, 2)
        .iter()
        .filter(|s| s.turns_saved >= 100)
        .count()
}

#[aoc(day20, part2)]
fn part2((input, start, end): &Parsed) -> usize {
    solve(*start, *end, input, 20)
        .iter()
        .filter(|s| s.turns_saved >= 100)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    const EXAMPLE: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

    #[test]
    fn part1_example() {
        let expected: HashMap<usize, Vec<(usize, usize)>> = HashMap::from([
            (64, vec![(0, 0); 1]),
            (40, vec![(0, 0); 1]),
            (38, vec![(0, 0); 1]),
            (36, vec![(0, 0); 1]),
            (20, vec![(0, 0); 1]),
            (12, vec![(0, 0); 3]),
            (10, vec![(0, 0); 2]),
            (8, vec![(0, 0); 4]),
            (6, vec![(0, 0); 2]),
            (4, vec![(0, 0); 14]),
            (2, vec![(0, 0); 14]),
        ]);

        let (data, start, end) = parse(EXAMPLE);

        let shortcuts = solve(start, end, &data, 2);

        let mut by_time = HashMap::new();

        for shortcut in shortcuts {
            by_time
                .entry(shortcut.turns_saved)
                .and_modify(|v: &mut Vec<_>| v.push(shortcut.clone()))
                .or_insert(vec![shortcut]);
        }

        for (time, positions) in by_time.iter() {
            let value = expected.get(time).unwrap();
            assert_eq!(positions.len(), value.len());
        }
    }
}
