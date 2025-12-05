use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Inventory {
    ranges: Vec<(usize, usize)>,
    ids: Vec<usize>,
}

impl Inventory {
    fn merge_ranges(&self) -> Vec<(usize, usize)> {
        if self.ranges.is_empty() {
            return vec![];
        }

        let mut sorted = self.ranges.clone();
        sorted.sort_by_key(|r| r.0);

        let mut merged = vec![sorted[0]];

        for &(start, end) in &sorted[1..] {
            let last_idx = merged.len() - 1;
            let (_, last_end) = merged[last_idx];

            // check if ranges overlap or are adjacent (end+1 == start)
            if start <= last_end + 1 {
                // overlaps or adjacent, merge by extending the end
                merged[last_idx].1 = end.max(last_end);
            } else {
                // no overlap, add as new range
                merged.push((start, end));
            }
        }

        merged
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Inventory {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let ids = ids.lines().map(|line| line.parse().unwrap()).collect();

    Inventory { ranges, ids }
}

#[aoc(day5, part1)]
fn part1(input: &Inventory) -> usize {
    input
        .ids
        .iter()
        .filter(|id| {
            for (start, end) in &input.ranges {
                if (start..=end).contains(id) {
                    return true;
                }
            }
            false
        })
        .count()
}

#[aoc(day5, part1, BinarySearch)]
fn part1_binary_search(input: &Inventory) -> usize {
    let merged_ranges = input.merge_ranges();
    input
        .ids
        .iter()
        .filter(|&&id| {
            merged_ranges
                .binary_search_by(|&(start, end)| {
                    if id < start {
                        std::cmp::Ordering::Greater
                    } else if id > end {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Equal
                    }
                })
                .is_ok()
        })
        .count()
}

#[aoc(day5, part2)]
fn part2(input: &Inventory) -> usize {
    input
        .merge_ranges()
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 14);
    }
}
