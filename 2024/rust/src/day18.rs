use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::DIRECTIONS4;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const GRID_SIZE: usize = {
    if cfg!(test) {
        7
    } else {
        71
    }
};

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            let x: usize = x.trim().parse().unwrap();
            let y: usize = y.trim().parse().unwrap();

            (x, y)
        })
        .collect()
}

// Helper function to check if a path exists
fn find_path(grid: &[Vec<u8>]) -> Option<(usize, Vec<(usize, usize)>)> {
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut previous: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut visited = vec![vec![false; GRID_SIZE]; GRID_SIZE];

    heap.push(State {
        cost: 0,
        position: (0, 0),
    });
    distances.insert((0, 0), 0);

    while let Some(State { position, cost }) = heap.pop() {
        let (x, y) = position;

        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;

        if position == (GRID_SIZE - 1, GRID_SIZE - 1) {
            // reconstruct path
            let mut path = vec![];
            let mut current = position;
            while let Some(p) = previous.get(&current) {
                path.push(*p);
                current = *p;
            }
            path.push((0, 0));

            return Some((cost, path));
        }

        for &(dx, dy) in &DIRECTIONS4 {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x < 0 || new_x >= GRID_SIZE as i32 || new_y < 0 || new_y >= GRID_SIZE as i32 {
                continue;
            }

            let new_pos = (new_x as usize, new_y as usize);

            if grid[new_pos.1][new_pos.0] == b'#' || visited[new_pos.1][new_pos.0] {
                continue;
            }

            heap.push(State {
                cost: distances[&position] + 1,
                position: new_pos,
            });
            distances.insert(new_pos, distances[&position] + 1);
            previous.insert(new_pos, position);
        }
    }

    None
}

#[aoc(day18, part1)]
fn part1(input: &[(usize, usize)]) -> usize {
    // let grid_size = *grid_size;
    let mut grid = vec![vec![b'.'; GRID_SIZE]; GRID_SIZE];

    // tests
    for &(x, y) in input.iter().take(match GRID_SIZE {
        7 => 12,
        71 => 1024,
        _ => unreachable!("Bad grid size"),
    }) {
        grid[y][x] = b'#';
    }

    let (cost, _) = find_path(&grid).expect("No path found");

    cost
}

#[aoc(day18, part2)]
fn part2(input: &[(usize, usize)]) -> String {
    let mut grid = vec![vec![b'.'; GRID_SIZE]; GRID_SIZE];

    // based off pt 1, we know that a valid input will have have a path after atleast this many obstacles so we can skip them
    for &(x, y) in input.iter().take(match GRID_SIZE {
        7 => 12,
        71 => 1024,
        _ => unreachable!("Bad grid size"),
    }) {
        grid[y][x] = b'#';
    }

    let mut last_path: Option<Vec<(usize, usize)>> = None;

    for &(x, y) in input.iter() {
        grid[y][x] = b'#';

        // if the last found path doesn't contain the new point, skip it
        if let Some(ref path) = last_path {
            if !path.iter().any(|&(px, py)| px == x && py == y) {
                continue;
            }
        }

        let Some((_, path)) = find_path(&grid) else {
            return format!("{},{}", x, y);
        };

        last_path = Some(path);
    }

    unreachable!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 22);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "6,1");
    }
}
