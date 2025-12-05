use aoc_runner_derive::aoc;

#[inline]
fn parse(input: &str) -> impl IntoIterator<Item = (usize, usize)> {
    input.split(',').map(|range| {
        let (start, end) = range.split_once('-').unwrap();
        let start: usize = start.parse().unwrap();
        let end: usize = end.parse().unwrap();

        (start, end)
    })
}

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    let mut total = 0;
    for (start, end) in parse(input) {
        let mut count = 0;
        for n in start..=end {
            if is_duplicated(n) {
                count += n;
            }
        }

        total += count;
    }

    total
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    let mut total = 0;
    for (start, end) in parse(input) {
        let mut count = 0;
        for n in start..=end {
            if is_repeated_pattern(n) {
                count += n;
            }
        }

        total += count;
    }

    total
}

fn is_duplicated(num: usize) -> bool {
    let digit_count = get_digit_count(num) as u32;

    // only makes sense for even digit counts
    if digit_count % 2 != 0 {
        return false;
    }

    let divisor = 10_i32.pow(digit_count / 2);
    let first_half = num / divisor as usize;
    let second_half = num % divisor as usize;

    first_half == second_half
}

fn is_repeated_pattern(num: usize) -> bool {
    // Count digits
    let digit_count = get_digit_count(num);

    // must divide evenly into total length
    // pattern has to repeat at least twice, so max length is digit_count / 2
    for pattern_len in 1..=(digit_count / 2) {
        if digit_count % pattern_len != 0 {
            continue; // doesn't divide evenly
        }

        let divisor = 10_i32.pow(pattern_len as u32) as usize;
        let pattern = num % divisor;

        // check if this pattern repeats throughout the entire number
        let mut remaining = num;
        let mut is_valid = true;

        while remaining > 0 {
            if remaining % divisor != pattern {
                is_valid = false;
                break;
            }
            remaining /= divisor;
        }

        if is_valid {
            return true;
        }
    }

    false
}

#[inline]
fn get_digit_count(mut num: usize) -> usize {
    let mut count = 0;
    while num > 0 {
        count += 1;
        num /= 10;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 1227775554);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 4174379265);
    }

    #[test]
    fn test_is_duplicated() {
        assert!(is_duplicated(1212));
        assert!(is_duplicated(123123));
        assert!(is_duplicated(99999999));

        assert!(!is_duplicated(1234));
        assert!(!is_duplicated(12345));
        assert!(!is_duplicated(123321));
        assert!(!is_duplicated(12345678));
    }
}
