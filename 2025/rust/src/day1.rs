#[aoc(day1, part1)]
fn part1(input: &[u8]) -> usize {
    let mut pos = 50;

    input.split(|b| b == &b'\n').fold(0, |mut acc, line| {
        let (direction, distance) = line.split_at(1);

        let distance = std::str::from_utf8(distance)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        match direction {
            b"L" => {
                pos = (pos - distance as i32 + 100) % 100;
            }
            b"R" => {
                pos = (pos + distance as i32) % 100;
            }
            _ => unreachable!(),
        }

        if pos == 0 {
            acc += 1;
        }

        acc
    })
}

#[aoc(day1, part2, Computed)]
fn part2_computed(input: &[u8]) -> u32 {
    let mut pos: i32 = 50;
    let mut answer = 0;

    for line in input.split(|b| b == &b'\n') {
        let (direction, distance) = line.split_at(1);
        let distance = std::str::from_utf8(distance)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        // count full rotations
        answer += (distance / 100) as u32;

        // check if remaining distance crosses 0 (but doesn't END at 0, and doesn't START at 0)
        let remaining = distance % 100;
        let new_pos = match direction {
            b"L" => {
                let new_pos = ((pos - distance) % 100 + 100) % 100;
                if remaining > pos && new_pos != 0 && pos != 0 {
                    answer += 1;
                }
                new_pos
            }
            b"R" => {
                let new_pos = (pos + distance) % 100;
                if remaining > (99 - pos) && new_pos != 0 {
                    answer += 1;
                }
                new_pos
            }
            _ => unreachable!(),
        };

        if new_pos == 0 {
            answer += 1;
        }

        pos = new_pos;
    }

    answer
}

#[aoc(day1, part2, Simulated)]
fn part2_simulated(input: &[u8]) -> u32 {
    let mut pos: i32 = 50;
    let mut answer = 0;

    for line in input.split(|b| b == &b'\n') {
        let (direction, distance) = line.split_at(1);
        let distance = std::str::from_utf8(distance)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        for _ in 0..distance {
            match direction {
                b"L" => {
                    pos -= 1;
                    if pos == -1 {
                        pos = 99;
                    }
                }
                b"R" => {
                    pos += 1;
                    if pos == 100 {
                        pos = 0;
                    }
                }
                _ => unreachable!(),
            }

            if pos == 0 {
                answer += 1;
            }
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE_INPUT.as_bytes()), 3);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2_computed(SAMPLE_INPUT.as_bytes()), 6);
        assert_eq!(part2_simulated(SAMPLE_INPUT.as_bytes()), 6);

        assert_eq!(part2_computed(b"R1000"), 10);
        assert_eq!(part2_simulated(b"R1000"), 10);
    }
}
