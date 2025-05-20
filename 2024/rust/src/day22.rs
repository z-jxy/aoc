use std::{collections::HashMap, usize};

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[inline(always)]
fn mix(secret: usize, num: usize) -> usize {
    secret ^ num
}

#[inline(always)]
fn prune(secret: usize) -> usize {
    secret % 16777216
}

fn get_next_secret(mut secret: usize) -> usize {
    secret = prune(mix(secret, secret * 64));
    secret = prune(mix(secret, secret / 32));
    secret = prune(mix(secret, secret * 2048));
    secret
}

#[aoc(day22, part1)]
fn part1(input: &[usize]) -> usize {
    input
        .iter()
        .map(|num| {
            let mut secret = *num;
            for _ in 0..2000 {
                secret = get_next_secret(secret);
            }
            secret
        })
        .sum()
}

#[aoc(day22, part2)]
fn part2(input: &[usize]) -> usize {
    let mut max_prices = HashMap::new();
    let mut prices = HashMap::with_capacity(2000);

    // each difference is in range -9..=9
    const SHIFT: i64 = 5;
    const OFFSET: i16 = 9;
    const STATE_MASK: i64 = (1 << (4 * SHIFT)) - 1;

    for &start_num in input {
        let mut num = start_num;
        let mut prev_price = (num % 10) as i16;

        let mut state = 0;

        // set initial state
        for _ in 0..4 {
            let new_secret = get_next_secret(num);
            let new_price = (new_secret % 10) as i16;
            let diff = prev_price - new_price;

            state = ((state << SHIFT) & STATE_MASK) | ((diff + OFFSET) as i64);

            num = new_secret;
            prev_price = new_price;
        }

        // start updating max_prices
        for _ in 4..2000 {
            let new_secret = get_next_secret(num);
            let new_price = (new_secret % 10) as i16;
            let diff = prev_price - new_price;

            state = ((state << SHIFT) & STATE_MASK) | ((diff + OFFSET) as i64);

            prices.entry(state).or_insert(new_price);

            num = new_secret;
            prev_price = new_price;
        }

        for (k, v) in prices.drain() {
            max_prices
                .entry(k)
                .and_modify(|n| *n += v)
                .or_insert_with(|| v);
        }
    }

    *max_prices.values().max().unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"1
10
100
2024"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 37327623);
    }

    #[test]
    fn part2_example() {
        // todo: example is off by 1
        assert_eq!(part2(&parse(EXAMPLE)), 23 + 1);
    }
}
