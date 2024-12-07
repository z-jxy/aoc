const fn precompute_powers_of_10() -> [usize; 18] {
    let mut p10 = [1; 18];
    let mut i = 1;
    while i < 18 {
        p10[i] = p10[i - 1] * 10;
        i += 1;
    }
    p10
}

const POWERS_OF_10: [usize; 18] = precompute_powers_of_10();

fn solve_p1(target: usize, numbers: &[usize]) -> bool {
    fn solve(target: usize, numbers: &[usize], index: usize, current: usize) -> bool {
        if current > target {
            return false;
        }
        if index == numbers.len() {
            return current == target;
        }

        // add
        if solve(target, numbers, index + 1, current + numbers[index]) {
            return true;
        }
        // multiply
        if solve(target, numbers, index + 1, current * numbers[index]) {
            return true;
        }

        false
    }

    solve(target, numbers, 1, numbers[0])
}

fn solve_p2(target: usize, numbers: &[usize]) -> bool {
    fn solve(target: usize, numbers: &[usize], index: usize, current: usize) -> bool {
        if current > target {
            return false;
        }
        if index == numbers.len() {
            return current == target;
        }

        // add
        if solve(target, numbers, index + 1, current + numbers[index]) {
            return true;
        }
        // multiply
        if solve(target, numbers, index + 1, current * numbers[index]) {
            return true;
        }

        // concatenate
        let d = (numbers[index] as f64).log10().floor() as usize + 1;
        let concatenated = current * POWERS_OF_10[d] + numbers[index];
        if solve(target, numbers, index + 1, concatenated) {
            return true;
        }

        false
    }

    solve(target, numbers, 1, numbers[0])
}

fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let target = parts.next().unwrap().parse::<usize>().unwrap();
            let values: Vec<usize> = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|part| part.parse::<usize>().unwrap())
                .collect();
            (target, values)
        })
        .collect::<Vec<_>>()
}

#[aoc::main(07)]
fn main(input: &str) -> (usize, usize) {
    let equations = parse_input(input);
    let (mut p1, mut p2) = (0, 0);
    for (target, values) in equations {
        match solve_p1(target, &values) {
            true => p1 += target,
            false if solve_p2(target, &values) => p2 += target,
            false => (),
        }
    }

    p2 += p1;

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test_p1() {
        let p1 = parse_input(EXAMPLE)
            .iter()
            .fold(0, |acc, (target, values)| {
                if solve_p1(*target, &values) {
                    acc + target
                } else {
                    acc
                }
            });
        assert_eq!(p1, 3749);
    }

    #[test]
    fn test_p2() {
        let p2 = parse_input(EXAMPLE)
            .iter()
            .fold(0, |acc, (target, values)| {
                if solve_p2(*target, &values) {
                    acc + target
                } else {
                    acc
                }
            });
        assert_eq!(p2, 11387);
    }
}
