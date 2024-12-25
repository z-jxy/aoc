mod day25;
mod day24;
mod day23;
mod day22;
mod day21;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day9;
#[macro_use]
extern crate aoc_runner_derive;
extern crate aoc_runner;

pub use aoc_macros::main;

/// Directions for 4-way movement (right, down, left, up)
pub const DIRECTIONS4: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub trait GridExt<S> {
    /// Returns the first unique position of the given identifier in the grid.
    ///
    /// (row, column)
    fn get_unique_position(&self, identifier: S) -> Option<(usize, usize)>;
}

impl<S: PartialEq> GridExt<S> for Vec<Vec<S>> {
    fn get_unique_position(&self, identifier: S) -> Option<(usize, usize)> {
        self.iter().enumerate().find_map(|(r, row)| {
            row.iter().enumerate().find_map(|(c, cell)| {
                if *cell == identifier {
                    Some((r, c))
                } else {
                    None
                }
            })
        })
    }
}

impl<S: PartialEq> GridExt<S> for [Vec<S>] {
    fn get_unique_position(&self, identifier: S) -> Option<(usize, usize)> {
        self.iter().enumerate().find_map(|(r, row)| {
            row.iter().enumerate().find_map(|(c, cell)| {
                if *cell == identifier {
                    Some((r, c))
                } else {
                    None
                }
            })
        })
    }
}

pub use GridExt as _;

aoc_lib! { year = 2024 }
