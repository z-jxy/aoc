use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Bank(Vec<u8>);

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Bank> {
    input
        .lines()
        .map(|line| {
            let digits = line.bytes().map(|b| b - b'0').collect::<Vec<u8>>();
            Bank(digits)
        })
        .collect()
}

fn highest_joltage(bank: &Bank) -> u8 {
    let n = bank.0.len();

    // precompute suffix maximums: suffix_max[i] = max(bank[i..])
    let mut suffix_max = vec![0u8; n];
    suffix_max[n - 1] = bank.0[n - 1];
    for i in (0..n - 1).rev() {
        suffix_max[i] = suffix_max[i + 1].max(bank.0[i]);
    }

    (0..n - 1)
        .map(|i| bank.0[i] * 10 + suffix_max[i + 1])
        .max()
        .unwrap()
}

fn highest_joltage_part2(bank: &Bank) -> u64 {
    const PICK: usize = 12;
    let n = bank.0.len();

    let mut result = 0u64;
    let mut current_pos = 0;
    let mut picked = 0;

    while picked < PICK {
        let remaining = PICK - picked;
        let search_end = n - (remaining - 1);

        let mut max_digit = 0u8;
        let mut best_pos = 0;

        for (i, &digit) in bank.0[current_pos..search_end].iter().enumerate() {
            if digit > max_digit {
                max_digit = digit;
                best_pos = i;
            }
        }

        result = result * 10 + max_digit as u64;
        current_pos = current_pos + best_pos + 1;
        picked += 1;
    }

    result
}

#[aoc(day3, part1)]
fn part1(input: &[Bank]) -> usize {
    input.iter().map(|b| highest_joltage(b) as usize).sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Bank]) -> usize {
    input
        .iter()
        .map(|b| highest_joltage_part2(b) as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 357);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 3121910778619);
    }
}
