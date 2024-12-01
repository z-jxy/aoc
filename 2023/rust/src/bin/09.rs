use itertools::Itertools;

fn find_next_value(nums: &Vec<i32>) -> i32 {
    let mut all_zeroes = true;
    let diffs = Itertools::tuple_windows(nums.iter())
        .map(|(x, y)| {
            let diff = y - x;
            all_zeroes = all_zeroes && diff == 0;
            diff
        })
        .collect();

    let end = *nums.last().expect("last");
    if all_zeroes {
        end
    } else {
        end + find_next_value(&diffs)
    }
}

fn parse_values(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|n| n.parse::<i32>().expect("n"))
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(parse_values).collect()
}

#[aoc::main(09)]
fn main(input: &str) -> (usize, usize) {
    let parsed = parse_input(input);

    // Part 1.
    println!("Part 1:");
    let sum_nexts: i32 = parsed.iter().map(|num| find_next_value(num)).sum();
    println!("Sum (forward): {sum_nexts}");

    // Part 2.
    println!("Part 2:");
    let reversed: Vec<Vec<i32>> = parsed
        .iter()
        .cloned()
        .map(|mut v| {
            v.reverse();
            v
        })
        .collect();
    let sum_prevs: i32 = reversed.iter().map(|num| find_next_value(num)).sum();
    println!("Sum (backward): {sum_prevs}");

    (0, 0)
}
