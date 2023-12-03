use std::collections::HashSet;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn extract_part(line: &Vec<char>, idx: usize) -> (u64, usize, usize) {
    let mut start = idx;
    while start > 0 && line[start - 1].is_digit(10) {
        start -= 1;
    }

    let end = idx + line[idx..].iter().take_while(|&&c| c.is_digit(10)).count();
    let part_number: String = line[start..end].iter().collect();

    (part_number.parse::<u64>().unwrap(), start, end)
}

fn check_adjacent_part(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    d_row: isize,
    d_col: isize,
    visited_positions: &mut HashSet<(usize, usize)>,
) -> Option<u64> {
    let new_row = row as isize + d_row;
    let new_col = col as isize + d_col;

    if new_row >= 0
        && new_col >= 0
        && new_row < grid.len() as isize
        && new_col < grid[new_row as usize].len() as isize
    {
        let target_row = new_row as usize;
        let target_col = new_col as usize;

        if !visited_positions.contains(&(target_row, target_col))
            && grid[target_row][target_col].is_digit(10)
        {
            let (part_number, start, end_idx) = extract_part(&grid[target_row], target_col);
            for idx in start..end_idx {
                visited_positions.insert((target_row, idx));
            }
            return Some(part_number);
        }
    }
    None
}

fn solve(grid: &Vec<Vec<char>>, is_part_two: bool) -> u64 {
    grid.iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(col, ch)| {
                    if is_part_two && *ch != '*' {
                        return None;
                    } else if *ch == '.' || !ch.is_ascii_punctuation() {
                        return None;
                    }
                    let mut parsed_positions: HashSet<(usize, usize)> = HashSet::new();
                    let part_numbers = &DIRECTIONS
                        .iter()
                        .filter_map(|&(dx, dy)| {
                            check_adjacent_part(&grid, row, col, dx, dy, &mut parsed_positions)
                        })
                        .collect::<Vec<u64>>();

                    if is_part_two {
                        if part_numbers.len() != 2 {
                            return None;
                        }
                        return Some(part_numbers.iter().product::<u64>());
                    }

                    return Some(part_numbers.iter().sum::<u64>());
                })
                .sum::<u64>()
        })
        .sum()
}

#[aoc::main(03)]
fn main(input: &str) -> (usize, usize) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let part_1 = solve(&grid, false);
    let part_2 = solve(&grid, true);

    (part_1 as usize, part_2 as usize)
}
