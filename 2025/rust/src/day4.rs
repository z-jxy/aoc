use aoc_runner_derive::{aoc, aoc_generator};

const MAX_ROLLS_ALLOWED: u8 = 4;
const DIRECTIONS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Vec<char>]) -> usize {
    let mut total = 0;

    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            match cell {
                '@' => {
                    let mut rolls = 0;

                    for &(dr, dc) in DIRECTIONS {
                        let r = row_idx as isize + dr;
                        let c = col_idx as isize + dc;

                        if r >= 0
                            && r < input.len() as isize
                            && c >= 0
                            && c < input[0].len() as isize
                        {
                            match input[r as usize][c as usize] {
                                '@' => {
                                    rolls += 1;
                                }
                                '.' => {}
                                _ => continue,
                            }
                        }
                    }

                    if rolls < MAX_ROLLS_ALLOWED {
                        total += 1;
                    }
                }
                '.' => {}
                _ => unreachable!(),
            }
        }
    }

    total
}

#[aoc(day4, part2)]
fn part2(input: &[Vec<char>]) -> usize {
    let mut total = 0;

    let mut input = input.to_vec();

    let rows = input.len();
    let cols = input[0].len();

    loop {
        let mut local_total = total;

        for row_idx in 0..rows {
            for col_idx in 0..cols {
                match input[row_idx][col_idx] {
                    '@' => {
                        let mut rolls = 0;

                        for &(dr, dc) in DIRECTIONS {
                            let r = row_idx as isize + dr;
                            let c = col_idx as isize + dc;

                            if r >= 0 && r < rows as isize && c >= 0 && c < cols as isize {
                                match input[r as usize][c as usize] {
                                    '@' => rolls += 1,
                                    _ => continue,
                                }
                            }
                        }

                        if rolls < MAX_ROLLS_ALLOWED {
                            local_total += 1;
                            input[row_idx][col_idx] = '.';
                        }
                    }
                    '.' => {}
                    _ => unreachable!(),
                }
            }
        }

        if local_total == total {
            break;
        }

        total = local_total;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 43);
    }
}
