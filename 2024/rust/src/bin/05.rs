use hashbrown::HashSet;
use itertools::Itertools;

fn p1(orderings: &HashSet<(usize, usize)>, updates: &[Vec<usize>]) -> usize {
    updates
        .iter()
        .filter(|update| {
            update
                .iter()
                .tuple_windows()
                .all(|(left, right)| !orderings.contains(&(*right, *left)))
        })
        .fold(0, |acc, update| acc + update[update.len() / 2])
}

fn p2(orderings: &HashSet<(usize, usize)>, updates: &mut [Vec<usize>]) -> usize {
    updates
        .iter_mut()
        .filter(|update| {
            update
                .iter()
                .tuple_windows()
                .any(|(left, right)| orderings.contains(&(*right, *left)))
        })
        .fold(0, |acc, update| {
            // reorder the updates
            let len = update.len();

            let mut swapped = true;
            while swapped {
                swapped = false; // assume no swaps needed
                for i in 1..len {
                    let left = update[i - 1];
                    let right = update[i];
                    if orderings.contains(&(right, left)) {
                        update.swap(i - 1, i);
                        swapped = true; // mark that a swap was made.
                    }
                }
            }

            acc + update[len / 2]
        })
}

fn parse(input: &str) -> (HashSet<(usize, usize)>, Vec<Vec<usize>>) {
    let mut split = input.split("\n\n");

    let ordering = split.next().unwrap();
    let pages = split.next().unwrap();

    let orderings = ordering
        .lines()
        .map(|line| {
            if let [left, right] = line
                .split("|")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>()[..]
            {
                (left, right)
            } else {
                unreachable!("Invalid input");
            }
        })
        .collect::<HashSet<_>>();

    let updates = pages
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (orderings, updates)
}

#[aoc::main(05)]
fn main(input: &str) -> (usize, usize) {
    let (orderings, mut updates) = parse(input);

    let p1 = p1(&orderings, &updates);
    let p2 = p2(&orderings, &mut updates);

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_p1() {
        let (orderings, updates) = parse(EXAMPLE);
        let p1 = p1(&orderings, &updates);
        assert_eq!(p1, 143);
    }

    #[test]
    fn test_p2() {
        let (orderings, mut updates) = parse(EXAMPLE);
        let p2 = p2(&orderings, &mut updates);
        assert_eq!(p2, 123);
    }
}
