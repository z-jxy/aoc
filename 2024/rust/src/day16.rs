use aoc_runner_derive::{aoc, aoc_generator};

use crate::GridExt;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

const TURN_COST: i32 = 1000;
const MOVE_COST: i32 = 1;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
struct Position {
    x: i32,
    y: i32,
}

// Add direction to track current movement
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    pos: Position,
    direction: (i32, i32), // None for start position
}

#[derive(Eq, PartialEq)]
struct Node {
    state: State,
    f_score: i32,
    g_score: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
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

impl Position {
    #[inline(always)]
    fn manhattan_distance(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn get_neighbors(state: &State, maze: &[Vec<u8>]) -> Vec<(State, i32)> {
    let mut neighbors = Vec::new();

    for &dir in DIRECTIONS.iter() {
        let new_x = state.pos.x + dir.0;
        let new_y = state.pos.y + dir.1;

        if new_x >= 0
            && new_x < maze[0].len() as i32
            && new_y >= 0
            && new_y < maze.len() as i32
            && maze[new_y as usize][new_x as usize] != b'#'
        {
            // Calculate cost based on whether we're turning
            let cost = match state.direction {
                // None => MOVE_COST,                                    // First move from start
                current_dir if current_dir == dir => MOVE_COST, // Same direction
                _ => TURN_COST + MOVE_COST,                     // Turn + move
            };

            let new_state = State {
                pos: Position { x: new_x, y: new_y },
                direction: dir,
            };

            neighbors.push((new_state, cost));
        }
    }
    neighbors
}

// Modified to return both path and total score
fn astar(maze: &[Vec<u8>], start: Position, goal: Position) -> Option<(Vec<Position>, i32)> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<State, State> = HashMap::new();
    let mut g_scores: HashMap<State, i32> = HashMap::new();

    let start_state = State {
        pos: start.clone(),
        direction: (1, 0), // Initially facing East
    };

    g_scores.insert(start_state.clone(), 0);
    open_set.push(Node {
        state: start_state.clone(),
        f_score: start.manhattan_distance(&goal),
        g_score: 0,
    });

    while let Some(current_node) = open_set.pop() {
        let current_state = current_node.state.clone();

        if current_state.pos == goal {
            let total_score = g_scores[&current_state];
            return Some((reconstruct_path(&came_from, current_state), total_score));
        }

        let current_g = g_scores[&current_state];

        for (neighbor_state, cost) in get_neighbors(&current_state, maze) {
            let tentative_g_score = current_g + cost;

            if !g_scores.contains_key(&neighbor_state)
                || tentative_g_score < g_scores[&neighbor_state]
            {
                came_from.insert(neighbor_state.clone(), current_state.clone());
                g_scores.insert(neighbor_state.clone(), tentative_g_score);
                let f_score = tentative_g_score + neighbor_state.pos.manhattan_distance(&goal);
                open_set.push(Node {
                    state: neighbor_state,
                    f_score,
                    g_score: tentative_g_score,
                });
            }
        }
    }

    None
}

fn best_tiles(
    maze: &Vec<Vec<u8>>,
    start: Position,
    goal: Position,
) -> Option<(Vec<Position>, i32, HashSet<Position>)> {
    // first pass: Find optimal path
    let (optimal_path, optimal_score) = astar(maze, start.clone(), goal.clone()).unwrap();

    let start_state = State {
        pos: start.clone(),
        direction: (1, 0),
    };

    // second pass: Find all tiles that are part of paths with optimal_score
    let mut open_set = BinaryHeap::new();
    let mut g_scores = HashMap::new();
    let mut best_tiles = HashSet::new();

    // track multiple possible previous states for each state
    let mut came_from: HashMap<State, HashSet<State>> = HashMap::new();

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
        if current_state.pos == goal && current_g == optimal_score {
            // use a DFS to find all paths back to start
            let mut stack = vec![(current_state.clone(), HashSet::new())];

            while let Some((state, mut path_tiles)) = stack.pop() {
                path_tiles.insert(state.pos.clone());

                if state.pos == start {
                    // found a complete path, add its tiles
                    best_tiles.extend(path_tiles);
                } else if let Some(prev_states) = came_from.get(&state) {
                    // add all possible previous states to explore
                    for prev_state in prev_states {
                        let new_path_tiles = path_tiles.clone();
                        stack.push((prev_state.clone(), new_path_tiles));
                    }
                }
            }
        }

        if current_g <= optimal_score {
            for (neighbor_state, cost) in get_neighbors(&current_state, maze) {
                let new_score = current_g + cost;

                if new_score <= optimal_score {
                    // allow multiple paths through the same state if they have the same score
                    if !g_scores.contains_key(&neighbor_state)
                        || new_score == g_scores[&neighbor_state]
                    {
                        came_from
                            .entry(neighbor_state.clone())
                            .or_insert_with(HashSet::new)
                            .insert(current_state.clone());

                        g_scores.insert(neighbor_state.clone(), new_score);
                        let new_f_score = new_score + neighbor_state.pos.manhattan_distance(&goal);
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

// Helper function to reconstruct path
#[inline(always)]
fn reconstruct_path(came_from: &HashMap<State, State>, mut current: State) -> Vec<Position> {
    let mut path = vec![current.pos];

    while let Some(prev_state) = came_from.get(&current) {
        path.push(prev_state.pos);
        current = prev_state.clone();
    }

    path.reverse();
    path
}

struct Input {
    maze: Vec<Vec<u8>>,
    start: Position,
    end: Position,
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Input {
    let maze = input
        .lines()
        .map(|line| line.bytes().collect())
        .collect::<Vec<_>>();

    let start = maze
        .get_unique_position(b'S')
        .map(|(y, x)| Position {
            x: x as i32,
            y: y as i32,
        })
        .unwrap();

    let end = maze
        .get_unique_position(b'E')
        .map(|(y, x)| Position {
            x: x as i32,
            y: y as i32,
        })
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
