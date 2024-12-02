use itertools::Itertools;

fn is_safe(x: &[i64]) -> bool {
    let mut ok = false;
    ok |= x.iter().tuple_windows().all(|(a, b)| a < b);
    ok |= x.iter().tuple_windows().all(|(a, b)| a > b);
    ok && x
        .iter()
        .tuple_windows()
        .all(|(a, b)| (1..=3).contains(&(a - b).abs()))
}

fn is_any_safe(x: &[i64]) -> bool {
    (0..x.len()).any(|i| {
        let mut y = x.to_vec();
        y.remove(i);
        is_safe(&y)
    })
}

#[aoc::main(02)]
fn main(input: &str) -> (usize, usize) {
    let (mut p1, mut p2) = (0, 0);
    for l in input.lines() {
        let x = l.split(' ').map(|w| w.parse().unwrap()).collect::<Vec<_>>();
        if is_safe(&x) {
            p1 += 1
        }
        if is_any_safe(&x) {
            p2 += 1
        }
    }
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn example_p1() {
        let result = EXAMPLE
            .lines()
            .map(|line| {
                line.split(" ")
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .filter(|x| is_safe(x))
            .count();

        assert_eq!(result, 2);
    }

    #[test]
    fn example_p2() {
        let result = EXAMPLE
            .lines()
            .map(|line| {
                line.split(" ")
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .filter(|x| is_any_safe(x))
            .count();

        assert_eq!(result, 4);
    }
}
