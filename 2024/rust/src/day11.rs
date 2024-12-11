use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn parse(input: &str) -> HashMap<usize, usize> {
    input
        .lines()
        .flat_map(|line| {
            line.split_whitespace()
                .map(|x| (x.parse::<usize>().unwrap(), 1))
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<usize, usize>>()
}

#[aoc(day11, part1)]
fn part1(input: &HashMap<usize, usize>) -> usize {
    let mut stone_counts = input.clone();

    let mut blink_ct = 0;
    while blink_ct < 25 {
        blink(&mut stone_counts);

        blink_ct += 1;
    }

    stone_counts.values().sum::<usize>()
}

#[aoc(day11, part2)]
fn part2(stones: &HashMap<usize, usize>) -> usize {
    let mut stone_counts = stones.clone();

    let mut blink_ct = 0;
    while blink_ct < 75 {
        blink(&mut stone_counts);
        blink_ct += 1;
    }

    stone_counts.values().sum::<usize>()
}

fn blink(stone_counts: &mut HashMap<usize, usize>) {
    let mut new_counts = HashMap::new();
    for (&stone, &count) in stone_counts.iter() {
        if stone == 0 {
            *new_counts.entry(1).or_insert(0) += count;
        } else if has_even_digits(stone) {
            if let Some((left, right)) = split_number_in_half(stone) {
                *new_counts.entry(left).or_insert(0) += count;
                *new_counts.entry(right).or_insert(0) += count;
            }
        } else {
            let new_value = stone * 2024;
            *new_counts.entry(new_value).or_insert(0) += count;
        }
    }
    *stone_counts = new_counts;
}

fn has_even_digits(number: usize) -> bool {
    let mut digit_count = 0;
    let mut num = number;
    while num > 0 {
        digit_count += 1;
        num /= 10;
    }
    digit_count % 2 == 0
}

fn split_number_in_half(number: usize) -> Option<(usize, usize)> {
    let digits = number.to_string();

    if digits.len() % 2 != 0 {
        return None;
    }

    let mid = digits.len() / 2;
    let first_half = digits[mid..].parse().unwrap();
    let second_half = digits[..mid].parse().unwrap();

    Some((first_half, second_half))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"125 17"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 55312);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 65601038650482);
    }
}
