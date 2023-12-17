use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

#[derive(Default, Clone, Copy, Debug)]
struct LightBeam {
    direction: Direction,
    position: (usize, usize),
}

impl std::ops::AddAssign<usize> for LightBeam {
    fn add_assign(&mut self, rhs: usize) {
        let (row, col) = self.position;
        match self.direction {
            Direction::Right => {
                self.position = (row, col + rhs);
            }
            Direction::Left => {
                self.position = (row, col - rhs);
            }
            Direction::Up => {
                self.position = (row - rhs, col);
            }
            Direction::Down => {
                self.position = (row + rhs, col);
            }
        }
    }
}

impl std::ops::Add<usize> for LightBeam {
    type Output = Self;

    fn add(mut self, rhs: usize) -> Self::Output {
        let (row, col) = self.position;
        match self.direction {
            Direction::Right => {
                self.position = (row, col + rhs);
            }
            Direction::Left => {
                self.position = (row, col - rhs);
            }
            Direction::Up => {
                self.position = (row - rhs, col);
            }
            Direction::Down => {
                self.position = (row + rhs, col);
            }
        }
        self
    }
}

impl<'a, T> std::cmp::PartialEq<T> for LightBeam
where
    T: AsRef<[&'a [u8]]>,
{
    fn eq(&self, other: &T) -> bool {
        let other_ref = other.as_ref();
        let (row, col) = self.position;
        row == other_ref.len() || col == other_ref[0].len()
    }
}

impl<'a, T> std::cmp::PartialOrd<T> for LightBeam
where
    T: AsRef<[&'a [u8]]>,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        let other_ref = other.as_ref();
        let (row, col) = self.position;
        match row >= other_ref.len() || col >= other_ref[0].len() {
            true => Some(Ordering::Greater),
            false => Some(Ordering::Less),
        }
    }
}

impl LightBeam {
    fn traverse(&mut self, grid: &[&[u8]]) -> usize {
        let mut energized_tiles: HashSet<(usize, usize)> = HashSet::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(*self);

        while let Some(mut beam) = queue.pop_front() {
            if visited.contains(&(beam.position, beam.direction)) || beam >= grid {
                continue;
            }
            energized_tiles.insert(beam.position);
            visited.insert((beam.position, beam.direction));

            let (row, col) = beam.position;
            let c = grid[row][col];
            let direction = &mut beam.direction;

            match c {
                b'/' => {
                    *direction = match direction {
                        Direction::Right => Direction::Up,
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                    };
                }
                b'\\' => {
                    *direction = match direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                }
                b'|' if *direction == Direction::Left || *direction == Direction::Right => {
                    beam.direction = Direction::Down;
                    if beam + 1 < grid {
                        let mut child = LightBeam {
                            direction: Direction::Up,
                            position: (row, col),
                        };
                        child += 1;
                        queue.push_back(child);
                    }
                }
                b'-' if *direction == Direction::Up || *direction == Direction::Down => {
                    beam.direction = Direction::Right;
                    if beam + 1 < grid {
                        let mut child = LightBeam {
                            direction: Direction::Left,
                            position: (row, col),
                        };
                        child += 1;
                        queue.push_back(child);
                    }
                }
                _ => {}
            };

            let mut f = beam;
            loop {
                f += 1;
                let (row, col) = f.position;
                if f >= grid || grid[row][col] != b'.' {
                    break;
                }
                energized_tiles.insert(f.position);
                visited.insert((f.position, f.direction));
            }

            if f < grid {
                queue.push_back(f);
            }
        }

        energized_tiles.len()
    }
}

fn p1(grid: &[&[u8]]) -> usize {
    let mut beam = LightBeam::default();
    let energized_tiles = beam.traverse(&grid);
    energized_tiles
}

fn p2(grid: &[&[u8]]) -> usize {
    let mut configurations = Vec::new();
    for i in 0..grid.len() {
        configurations.push(LightBeam {
            direction: Direction::Right,
            position: (i, 0),
        });
        configurations.push(LightBeam {
            direction: Direction::Left,
            position: (i, grid[0].len() - 1),
        });
    }

    for i in 0..grid[0].len() {
        configurations.push(LightBeam {
            direction: Direction::Down,
            position: (0, i),
        });
        configurations.push(LightBeam {
            direction: Direction::Up,
            position: (grid.len() - 1, i),
        });
    }

    let max = configurations
        .iter_mut()
        .map(|beam| beam.traverse(&grid))
        .max()
        .unwrap();

    max
}

mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let example = std::fs::read_to_string("../inputs/16.test").unwrap();
        assert_eq!(
            46,
            p1(&example.lines().map(|l| l.as_bytes()).collect::<Vec<_>>())
        );
        let input = std::fs::read_to_string("../inputs/16.txt").unwrap();
        assert_eq!(
            7543,
            p1(&input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>())
        );
    }

    #[test]
    fn test_p2() {
        let example = std::fs::read_to_string("../inputs/16.test").unwrap();
        assert_eq!(
            51,
            p2(&example.lines().map(|l| l.as_bytes()).collect::<Vec<_>>())
        );
        let input = std::fs::read_to_string("../inputs/16.txt").unwrap();
        assert_eq!(
            8231,
            p2(&input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>())
        );
    }
}

#[aoc::main(16)]
fn main(input: &str) -> (usize, usize) {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    (p1(&grid), p2(&grid))
}
