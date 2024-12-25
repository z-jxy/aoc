use aoc_runner_derive::{aoc, aoc_generator};

struct Key(Vec<Vec<u8>>);
impl Key {
    // same as lock, but we count from the bottom row
    fn pin_height(&self) -> Vec<usize> {
        let mut col_heights = vec![0; self.0[0].len()];

        for (i, col) in (0..self.0[0].len()).enumerate() {
            let mut height = 0;
            for row in (0..self.0.len()).rev() {
                if self.0[row][col] == b'#' {
                    height += 1;
                }
            }
            col_heights[i] = height - 1; // account for bottom row
        }

        col_heights
    }
}

struct Lock(Vec<Vec<u8>>);

impl Lock {
    fn pin_height(&self) -> Vec<usize> {
        let mut col_heights = vec![0; self.0[0].len()];

        for (i, col) in (0..self.0[0].len()).enumerate() {
            let mut height = 0;
            for row in 0..self.0.len() {
                if self.0[row][col] == b'#' {
                    height += 1;
                }
            }
            col_heights[i] = height - 1; // account for top row
        }

        col_heights
    }
}

type Input = (Vec<Key>, Vec<Lock>);

fn unlocks(key: &Key, lock: &Lock) -> bool {
    let pin_heights = lock.pin_height();
    let key_heights = key.pin_height();

    let lock_col_bounds = lock.0.len();

    for (lock, key) in pin_heights.iter().zip(key_heights.iter()) {
        if (*lock + 1) + (*key + 1) > lock_col_bounds {
            return false;
        }
    }

    true
}

#[aoc_generator(day25)]
fn parse(input: &str) -> Input {
    let mut input = input.split("\n\n");

    let (mut keys, mut locks) = (Vec::new(), Vec::new());

    while let Some(next) = input.next() {
        let griddy: Vec<Vec<_>> = next.lines().map(|l| l.bytes().collect()).collect();
        match griddy[0].iter().all(|&b| b == b'#') {
            true => locks.push(Lock(griddy)),
            false => keys.push(Key(griddy)),
        }
    }

    (keys, locks)
}

#[aoc(day25, part1)]
fn part1((keys, locks): &Input) -> usize {
    let mut unique = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if unlocks(key, lock) {
                unique += 1;
            }
        }
    }
    unique
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3);
    }
}
