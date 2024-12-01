fn p1(mut left: Vec<usize>, mut right: Vec<usize>) -> usize {
    let mut total_distance = 0;

    left.sort();
    right.sort();

    for i in 0..left.len() {
        total_distance += std::cmp::max(left[i], right[i]) - std::cmp::min(left[i], right[i]);
    }

    total_distance
}

fn p2(left: &[usize], right: &[usize]) -> usize {
    let mut similarity = 0;
    let mut right_counter = std::collections::HashMap::new();

    for i in 0..right.len() {
        right_counter
            .entry(right[i])
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    for i in 0..left.len() {
        if let Some(r) = right_counter.get(&left[i]) {
            similarity += left[i] * r;
        }
    }

    similarity
}

#[aoc::main(01)]
fn main(input: &str) -> (usize, usize) {
    let mut left = vec![];
    let mut right = vec![];

    input.lines().for_each(|line| {
        let split = line.split("   ").collect::<Vec<&str>>();
        left.push(split[0].parse::<usize>().unwrap());
        right.push(split[1].parse::<usize>().unwrap());
    });

    let p2 = p2(&left, &right);
    let p1 = p1(left, right);

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn example_p1() {
        let mut left = vec![];
        let mut right = vec![];

        EXAMPLE.lines().for_each(|line| {
            let split = line.split("   ").collect::<Vec<&str>>();
            left.push(split[0].parse::<usize>().unwrap());
            right.push(split[1].parse::<usize>().unwrap());
        });

        let sum = p1(left, right);

        assert_eq!(sum, 11);
    }

    #[test]
    fn example_p2() {
        let mut left = vec![];
        let mut right = vec![];

        EXAMPLE.lines().for_each(|line| {
            let split = line.split("   ").collect::<Vec<&str>>();
            left.push(split[0].parse::<usize>().unwrap());
            right.push(split[1].parse::<usize>().unwrap());
        });

        let p2 = p2(&left, &right);

        assert_eq!(p2, 31);
    }
}
