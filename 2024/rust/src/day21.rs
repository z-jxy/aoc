use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Key(isize, isize);

impl Key {
    // shared
    const A: Key = Self(0, 0);

    // numeric keypad
    const ZERO: Key = Self(1, 0);
    const ONE: Key = Self(2, 1);
    const TWO: Key = Self(1, 1);
    const THREE: Key = Self(0, 1);
    const FOUR: Key = Self(2, 2);
    const FIVE: Key = Self(1, 2);
    const SIX: Key = Self(0, 2);
    const SEVEN: Key = Self(2, 3);
    const EIGHT: Key = Self(1, 3);
    const NINE: Key = Self(0, 3);

    // directional keypad
    const ARROW_UP: Key = Self(1, 0);
    const ARROW_RIGHT: Key = Self(0, -1);
    const ARROW_DOWN: Key = Self(1, -1);
    const ARROW_LEFT: Key = Self(2, -1);
}

impl From<Key> for usize {
    fn from(key: Key) -> Self {
        match key {
            Key::ZERO => 0,
            Key::ONE => 1,
            Key::TWO => 2,
            Key::THREE => 3,
            Key::FOUR => 4,
            Key::FIVE => 5,
            Key::SIX => 6,
            Key::SEVEN => 7,
            Key::EIGHT => 8,
            Key::NINE => 9,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for Key {
    fn from(num: u8) -> Self {
        match num {
            b'A' => Self::A,
            b'0' => Self::ZERO,
            b'1' => Self::ONE,
            b'2' => Self::TWO,
            b'3' => Self::THREE,
            b'4' => Self::FOUR,
            b'5' => Self::FIVE,
            b'6' => Self::SIX,
            b'7' => Self::SEVEN,
            b'8' => Self::EIGHT,
            b'9' => Self::NINE,
            _ => unreachable!(),
        }
    }
}

fn get_conut(cur: &[Key], depth: u8, memo: &mut HashMap<(Vec<Key>, u8), usize>) -> usize {
    if depth == 0 {
        return cur.len();
    }

    let cache_key = (cur.to_vec(), depth);
    if let Some(&result) = memo.get(&cache_key) {
        return result;
    }

    let mut prev = Key::A;
    let mut count = 0;

    for &key in cur {
        let mut a = vec![];

        if prev.0 > key.0 {
            a.extend(vec![Key::ARROW_RIGHT; prev.0.abs_diff(key.0)]);
        }

        if prev.1 < key.1 {
            a.extend(vec![Key::ARROW_UP; prev.1.abs_diff(key.1)]);
        }

        if prev.1 > key.1 {
            a.extend(vec![Key::ARROW_DOWN; prev.1.abs_diff(key.1)]);
        }

        if prev.0 < key.0 {
            a.extend(vec![Key::ARROW_LEFT; prev.0.abs_diff(key.0)]);
        }

        a.push(Key::A);

        let mut count_a = get_conut(&a, depth - 1, memo);

        if !(prev.1 == 0 && key.0 == 2 || key.1 == 0 && prev.0 == 2) {
            a.pop();
            a.reverse();
            a.push(Key::A);

            let count_b = get_conut(&a, depth - 1, memo);
            if count_a > count_b {
                count_a = count_b;
            }
        }

        prev = key;
        count += count_a;
    }

    memo.insert(cache_key, count);
    count
}

fn solve(input: &[Vec<Key>], depth: u8) -> usize {
    let mut memo = HashMap::new();

    let complexity = input
        .iter()
        .map(|input| {
            let mut numeric = 0;
            input[..input.len() - 1].iter().for_each(|c| {
                numeric *= 10;
                numeric += usize::from(*c);
            });
            numeric * get_conut(input, depth, &mut memo)
        })
        .sum();

    complexity
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Vec<Vec<Key>> {
    input
        .lines()
        .map(|line| line.bytes().map(Into::into).collect())
        .collect()
}

#[aoc(day21, part1)]
fn part1(input: &Vec<Vec<Key>>) -> usize {
    solve(input, 3)
}

#[aoc(day21, part2)]
fn part2(input: &Vec<Vec<Key>>) -> usize {
    solve(input, 26)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"029A
980A
179A
456A
379A"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 126384);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 154115708116294);
    }
}
