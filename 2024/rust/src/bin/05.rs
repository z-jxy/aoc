use hashbrown::HashSet;
use itertools::Itertools;

fn p1(orderings: &HashSet<(usize, usize)>, updates: &[Vec<usize>]) -> usize {
    let mut valid_updates = vec![];
    for update in updates {
        let tuple_windows = update.iter().tuple_windows();
        let mut valid = true;

        for (left, right) in tuple_windows {
            if orderings.contains(&(*right, *left)) {
                valid = false;
                break;
            }
        }

        if valid {
            valid_updates.push(update);
        }
    }

    valid_updates
        .iter()
        .fold(0, |acc, update| acc + update[update.len() / 2])
}

fn p2(orderings: &HashSet<(usize, usize)>, updates: &[Vec<usize>]) -> usize {
    // let mut valid_updates = vec![];
    let mut invalid_updates = vec![];
    for update in updates {
        let tuple_windows = update.iter().tuple_windows();
        let mut valid = true;

        for (left, right) in tuple_windows {
            if orderings.contains(&(*right, *left)) {
                valid = false;
                break;
            }
        }

        if !valid {
            invalid_updates.push(update);
        }
    }

    let mut fixed_updates = vec![];
    for update in invalid_updates.iter_mut() {
        let len = update.len();
        let mut buf = update.clone(); // Start with a copy of the update.

        let mut swapped = true;
        while swapped {
            swapped = false; // Assume no swaps will be needed this pass.

            for i in 1..len {
                let left = buf[i - 1];
                let right = buf[i];

                if orderings.contains(&(right, left)) {
                    // Swap them
                    buf[i - 1] = right;
                    buf[i] = left;
                    swapped = true; // Mark that a swap was made.
                }
            }
        }

        fixed_updates.push(buf); // Store the fixed update.
    }

    fixed_updates
        .iter()
        .fold(0, |acc, update| acc + update[update.len() / 2])
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
                panic!("Invalid input")
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
    let (orderings, updates) = parse(input);

    let p1 = p1(&orderings, &updates);
    let p2 = p2(&orderings, &updates);

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
        let (orderings, updates) = parse(EXAMPLE);
        let p2 = p2(&orderings, &updates);
        assert_eq!(p2, 123);
    }
}
