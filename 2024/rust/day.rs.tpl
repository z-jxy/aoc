use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator({DAY})]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc({DAY}, part1)]
fn part1(input: &str) -> usize {
    0
}

#[aoc({DAY}, part2)]
fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"<EXAMPLE>"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(&EXAMPLE)), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(&EXAMPLE)), 0);
    }
}
