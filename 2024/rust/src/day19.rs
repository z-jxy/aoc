use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day19)]
fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let (fabrics, designs) = input.split_once("\n\n").unwrap();

    let mut fabrics = fabrics
        .split(", ")
        .map(|f| f.to_string())
        .collect::<Vec<String>>();

    fabrics.sort_by(|a, b| b.len().cmp(&a.len()));

    let designs = designs
        .lines()
        .map(|d| d.to_string())
        .collect::<Vec<String>>();

    (fabrics, designs)
}

#[inline(always)]
fn is_valid_pattern(pattern: &str, design: &str) -> bool {
    pattern.len() <= design.len() && design.starts_with(pattern)
}

fn is_valid_design(fabrics: &[String], design: &str, memo: &mut HashMap<String, bool>) -> bool {
    if design.is_empty() {
        return true;
    }

    if let Some(&result) = memo.get(design) {
        return result;
    }

    let result = fabrics.iter().any(|pattern| {
        is_valid_pattern(pattern, design)
            && is_valid_design(fabrics, &design[pattern.len()..], memo)
    });

    memo.insert(design.to_string(), result);
    result
}

fn count(fabrics: &[String], design: &str, memo: &mut HashMap<String, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&count) = memo.get(design) {
        return count;
    }

    let count = fabrics
        .iter()
        .filter(|pattern| is_valid_pattern(pattern, design))
        .map(|pattern| count(fabrics, &design[pattern.len()..], memo))
        .sum();

    memo.insert(design.to_string(), count);
    count
}

#[aoc(day19, part1)]
fn part1((fabrics, designs): &(Vec<String>, Vec<String>)) -> usize {
    let mut memo = HashMap::new();
    designs
        .iter()
        .filter(|design| is_valid_design(fabrics, design, &mut memo))
        .count()
}

#[aoc(day19, part2)]
fn part2((fabrics, designs): &(Vec<String>, Vec<String>)) -> usize {
    let mut memo = HashMap::new();
    designs
        .iter()
        .map(|design| count(fabrics, design, &mut memo))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 16);
    }
}
