use aoc_runner_derive::{aoc, aoc_generator};

use crate::GridExt;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

const TURN_COST: i32 = 1000;
const MOVE_COST: i32 = 1;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct PackedPosition(u32);

impl PackedPosition {
    #[inline(always)]
    fn new(x: i32, y: i32) -> Self {
        debug_assert!(x >= 0 && x < 65536 && y >= 0 && y < 65536);
        PackedPosition(((y as u32) << 16) | (x as u32))
    }

    #[inline(always)]
    fn x(self) -> i32 {
        (self.0 & 0xFFFF) as i32
    }

    #[inline(always)]
    fn y(self) -> i32 {
        (self.0 >> 16) as i32
    }

    #[inline(always)]
    fn manhattan_distance(self, other: &PackedPosition) -> i32 {
        (self.x() - other.x()).abs() + (self.y() - other.y()).abs()
    }
}

// track current movement
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct PackedState(u32);

impl PackedState {
    #[inline(always)]
    fn new(pos: PackedPosition, dir: (i32, i32)) -> Self {
        let dir_bits = match dir {
            (0, 1) => 0,  // right
            (1, 0) => 1,  // down
            (0, -1) => 2, // left
            (-1, 0) => 3, // up
            _ => unreachable!(),
        };
        // Pack position and direction into a single u32
        PackedState(pos.0 | (dir_bits << 30))
    }

    #[inline(always)]
    fn pos(self) -> PackedPosition {
        PackedPosition(self.0 & 0x3FFFFFFF) // Mask off direction bits
    }

    #[inline(always)]
    fn direction(self) -> (i32, i32) {
        match (self.0 >> 30) & 0x3 {
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, -1),
            3 => (-1, 0),
            _ => unreachable!(),
        }
    }
}

#[derive(Eq, PartialEq)]
struct Node {
    state: PackedState,
    f_score: i32,
    g_score: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse ordering for min-heap
        other
            .f_score
            .cmp(&self.f_score)
            .then_with(|| other.g_score.cmp(&self.g_score))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbors(state: &PackedState, maze: &[Vec<u8>]) -> [Option<(PackedState, i32)>; 4] {
    let mut neighbors: [Option<(PackedState, i32)>; 4] = [None; 4];

    for (i, &dir) in DIRECTIONS.iter().enumerate() {
        let new_x = state.pos().x() + dir.0;
        let new_y = state.pos().y() + dir.1;

        if new_x >= 0
            && new_x < maze[0].len() as i32
            && new_y >= 0
            && new_y < maze.len() as i32
            && maze[new_y as usize][new_x as usize] != b'#'
        {
            // calculate cost based on whether we're turning
            let cost = match state.direction() {
                current_dir if current_dir == dir => MOVE_COST, // same direction
                _ => TURN_COST + MOVE_COST,                     // turn + move
            };

            let new_state = PackedState::new(PackedPosition::new(new_x, new_y), dir);
            neighbors[i] = Some((new_state, cost));
        }
    }

    neighbors
}

fn astar(
    maze: &[Vec<u8>],
    start: PackedPosition,
    goal: PackedPosition,
) -> Option<(Vec<PackedPosition>, i32)> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<PackedState, PackedState> = HashMap::new();
    let mut g_scores: HashMap<PackedState, i32> = HashMap::new();

    let start_state = PackedState::new(start, (1, 0));

    g_scores.insert(start_state, 0);
    open_set.push(Node {
        state: start_state,
        f_score: start.manhattan_distance(&goal),
        g_score: 0,
    });

    while let Some(current_node) = open_set.pop() {
        let current_state = current_node.state;

        if current_state.pos() == goal {
            let total_score = g_scores[&current_state];
            return Some((reconstruct_path(&came_from, current_state), total_score));
        }

        let current_g = g_scores[&current_state];

        for (neighbor_state, cost) in get_neighbors(&current_state, maze)
            .iter()
            .filter_map(|n| *n)
        {
            let tentative_g_score = current_g + cost;

            if !g_scores.contains_key(&neighbor_state)
                || tentative_g_score < g_scores[&neighbor_state]
            {
                came_from.insert(neighbor_state, current_state);
                g_scores.insert(neighbor_state, tentative_g_score);
                open_set.push(Node {
                    state: neighbor_state,
                    f_score: tentative_g_score + neighbor_state.pos().manhattan_distance(&goal),
                    g_score: tentative_g_score,
                });
            }
        }
    }

    None
}

fn best_tiles(
    maze: &Vec<Vec<u8>>,
    start: PackedPosition,
    goal: PackedPosition,
) -> Option<(Vec<PackedPosition>, i32, HashSet<PackedPosition>)> {
    // first pass: Find optimal path
    let (optimal_path, optimal_score) = astar(maze, start.clone(), goal.clone()).unwrap();

    let start_state = PackedState::new(start, (1, 0));

    // second pass: Find all tiles that are part of paths with optimal_score
    let mut open_set = BinaryHeap::new();
    let mut g_scores = HashMap::new();
    let mut best_tiles = HashSet::new();

    // track multiple possible previous states for each state
    let mut came_from: HashMap<PackedState, HashSet<PackedState>> = HashMap::new();

    g_scores.insert(start_state.clone(), 0);
    open_set.push(Node {
        state: start_state.clone(),
        f_score: start.manhattan_distance(&goal),
        g_score: 0,
    });

    // find all valid paths
    while let Some(current_node) = open_set.pop() {
        let current_state = current_node.state;
        let current_g = g_scores[&current_state];

        // if at goal with optimal score, reconstruct all possible paths
        if current_state.pos() == goal && current_g == optimal_score {
            let mut stack = vec![(current_state, HashSet::new())];

            while let Some((state, mut path_tiles)) = stack.pop() {
                path_tiles.insert(state.pos());

                if state.pos() == start {
                    // found a complete path, add its tiles
                    best_tiles.extend(path_tiles);
                } else if let Some(prev_states) = came_from.get(&state) {
                    // add all possible previous states to explore
                    for prev_state in prev_states {
                        let new_path_tiles = path_tiles.clone();
                        stack.push((*prev_state, new_path_tiles));
                    }
                }
            }
        }

        if current_g <= optimal_score {
            for (neighbor_state, cost) in get_neighbors(&current_state, maze)
                .iter()
                .filter_map(|n| *n)
            {
                let new_score = current_g + cost;

                if new_score <= optimal_score {
                    // allow multiple paths through the same state if they have the same score
                    if !g_scores.contains_key(&neighbor_state)
                        || new_score == g_scores[&neighbor_state]
                    {
                        came_from
                            .entry(neighbor_state)
                            .or_insert_with(HashSet::new)
                            .insert(current_state);

                        g_scores.insert(neighbor_state, new_score);
                        let new_f_score =
                            new_score + neighbor_state.pos().manhattan_distance(&goal);
                        open_set.push(Node {
                            state: neighbor_state,
                            f_score: new_f_score,
                            g_score: new_score,
                        });
                    }
                }
            }
        }
    }

    Some((optimal_path, optimal_score, best_tiles))
}

#[inline(always)]
fn reconstruct_path(
    came_from: &HashMap<PackedState, PackedState>,
    mut current: PackedState,
) -> Vec<PackedPosition> {
    let mut path = vec![current.pos()];

    while let Some(prev_state) = came_from.get(&current) {
        path.push(prev_state.pos());
        current = *prev_state;
    }

    path.reverse();
    path
}

struct Input {
    maze: Vec<Vec<u8>>,
    start: PackedPosition,
    end: PackedPosition,
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Input {
    let maze = input
        .lines()
        .map(|line| line.bytes().collect())
        .collect::<Vec<_>>();

    let start = maze
        .get_unique_position(b'S')
        .map(|(y, x)| PackedPosition::new(x as i32, y as i32))
        .unwrap();

    let end = maze
        .get_unique_position(b'E')
        .map(|(y, x)| PackedPosition::new(x as i32, y as i32))
        .unwrap();

    Input { maze, start, end }
}

#[aoc(day16, part1)]
fn part1(input: &Input) -> usize {
    let (_, score) = astar(&input.maze, input.start, input.end).unwrap();

    score as usize
}

#[aoc(day16, part2)]
fn part2(input: &Input) -> usize {
    let (_, _, best_tiles) = best_tiles(&input.maze, input.start, input.end).unwrap();

    best_tiles.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 7036);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 45);
    }
}
