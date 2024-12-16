mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day9;
#[macro_use]
extern crate aoc_runner_derive;
extern crate aoc_runner;

pub use aoc_macros::main;

pub trait GridExt<S> {
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
