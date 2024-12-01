use std::collections::btree_set::SymmetricDifference;

fn is_vertical_symmetry(grid: &[Vec<char>], start_col: usize, end_col: usize) -> bool {
    for row in grid {
        if row[start_col] != row[end_col] {
            return false;
        }
    }
    true
}

fn is_horizontal_symmetry(grid: &[Vec<char>], top_row: usize, bottom_row: usize) -> bool {
    for col in 0..grid[0].len() {
        if grid[top_row][col] != grid[bottom_row][col] {
            return false;
        }
    }
    true
}

fn find_reflection_point(grid: &[Vec<char>]) -> usize {
    // Check for vertical symmetry
    let num_cols = grid[0].len();
    let num_rows = grid.len();

    for mid_col in 0..num_cols - 1 {
        if is_vertical_symmetry(grid, mid_col, mid_col + 1) {
            let mut left_col = mid_col;
            let mut right_col = mid_col + 1;

            println!(
                "vertical symmetry between columns {} and {}",
                left_col, right_col
            );

            while left_col > 0
                && right_col < num_cols - 1
                && is_vertical_symmetry(grid, left_col - 1, right_col + 1)
            {
                left_col -= 1;
                right_col += 1;
            }

            println!("left_col: {}, right_col: {}", left_col, right_col);

            // Ensure the reflection spans at least two columns
            if right_col - left_col > 1 {
                println!(
                    " reflection between columns {} and {} | {} {}",
                    left_col,
                    right_col,
                    mid_col + 1,
                    mid_col + 2
                );
                // factor is the total number of columns to the left of the reflection point
                let factor = mid_col + 1;
                return factor;

                // edge case: if the reflection point is at the left of the grid, return 1
            } else if right_col - left_col == 1 {
                if left_col == 0 && right_col == 1 {
                    println!("[first] num_cols: {}", num_cols - 1);
                    return 1;
                } else if right_col == num_cols - 1 {
                    println!("[last] num_cols: {}", num_cols - 1);
                    return mid_col + 1;
                }
            }
        }
    }

    for mid_row in 0..num_rows - 1 {
        if is_horizontal_symmetry(grid, mid_row, mid_row + 1) {
            let mut top_row = mid_row;
            let mut bottom_row = mid_row + 1;

            println!(
                "horizontal symmetry between rows {} and {}",
                top_row, bottom_row
            );

            while top_row > 0
                && bottom_row < num_rows - 1
                && is_horizontal_symmetry(grid, top_row - 1, bottom_row + 1)
            {
                top_row -= 1;
                bottom_row += 1;
            }

            println!(
                " reflection between rows {} and {} | {} {}\n---",
                top_row,
                bottom_row,
                mid_row + 1,
                mid_row + 2
            );

            // Ensure the reflection spans at least two rows
            if bottom_row - top_row > 1 {
                println!(
                    "[clean] reflection between rows {} and {} | {} {}\n---",
                    top_row,
                    bottom_row,
                    mid_row + 1,
                    mid_row + 2
                );
                // factor is the total number of rows above the reflection point * 100
                let factor = (mid_row + 1) * 100;
                return factor;

                // edge case: if the reflection point is at the top of the grid, return 100
            } else if bottom_row - top_row == 1 {
                if top_row == 0 && bottom_row == 1 {
                    return 100;
                } else if bottom_row == num_rows - 1 {
                    println!(" num_rows: {}", num_rows - 1);
                    return (mid_row + 1) * 100;
                }
            }
        }
    }

    println!("---");

    panic!("No reflection found");
}

mod part_one {
    use super::*;

    #[allow(unused)]
    fn part_one() -> usize {
        parse_input().into_iter().map(calc_group).sum::<usize>()
    }

    #[cfg(test)]
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 30705);
    }
}

mod part_two {
    use super::*;

    #[allow(unused)]
    fn part_two() -> usize {
        parse_input()
            .into_iter()
            .map(calc_group_smudged)
            .sum::<usize>()
    }

    #[cfg(test)]
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 44615);
    }
}

type Index = usize;
type AB = (Index, Index);

type Row = Vec<Cell>;
type Group = Vec<Row>;
type Input = Vec<Group>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Ash,
    Rock,
}

fn check_full_col_reflection(group: &Group, ab: AB) -> Option<usize> {
    check_full_reflection(group, ab, get_column, width)
}

fn check_full_row_reflection(group: &Group, ab: AB) -> Option<usize> {
    check_full_reflection(group, ab, get_row, height)
}

fn check_full_reflection(
    group: &Group,
    (initial_a, initial_b): AB,
    get_row_or_col: impl Fn(&Group, Index) -> Vec<Cell>,
    upper_limit: impl Fn(&Group) -> usize,
) -> Option<usize> {
    fn check_reflection(
        group: &Group,
        (a, b): AB,
        get_row_or_col: impl Fn(&Group, Index) -> Vec<Cell>,
    ) -> bool {
        get_row_or_col(group, a) == get_row_or_col(group, b)
    }

    let (mut a, mut b) = (Some(initial_a), Some(initial_b));

    loop {
        match (a, b) {
            (None, None) => unreachable!(),
            (Some(_a), Some(_b)) => {
                if !check_reflection(group, (_a, _b), &get_row_or_col) {
                    return None;
                }

                (a, b) = move_away_indeces((_a, _b), 1, upper_limit(group));
            }
            (Some(_), None) | (None, Some(_)) => {
                return Some(initial_a + 1);
            }
        }
    }
}

fn calc_group(group: Group) -> usize {
    let col_indeces = generate_initital_column_indeces(&group);

    let col_reflection = col_indeces
        .into_iter()
        .find_map(|(l, r)| check_full_col_reflection(&group, (l, r)));

    let row_indeces = generate_initial_row_indeces(&group);

    let row_reflection = row_indeces
        .into_iter()
        .find_map(|(l, r)| check_full_row_reflection(&group, (l, r)));

    col_reflection.unwrap_or(0) + row_reflection.unwrap_or(0) * 100
}

fn check_full_col_reflection_smudged(group: &Group, ab: AB) -> Option<usize> {
    check_full_reflection_smudged(group, ab, get_column, width)
}

fn check_full_row_reflection_smudged(group: &Group, ab: AB) -> Option<usize> {
    check_full_reflection_smudged(group, ab, get_row, height)
}

fn check_full_reflection_smudged(
    group: &Group,
    (initial_a, initial_b): AB,
    get_row_or_col: impl Fn(&Group, Index) -> Vec<Cell>,
    upper_limit: impl Fn(&Group) -> usize,
) -> Option<usize> {
    type FixedSmuged = bool;
    fn check_reflection_smudged(
        group: &Group,
        (a, b): AB,
        get_row_or_col: impl Fn(&Group, Index) -> Vec<Cell>,
    ) -> (bool, FixedSmuged) {
        let a = get_row_or_col(group, a);
        let b = get_row_or_col(group, b);

        let eq = a
            .clone()
            .into_iter()
            .zip(b.into_iter())
            .filter(|(a, b)| a == b)
            .count();

        if eq == a.len() {
            (true, false)
        } else if eq + 1 == a.len() {
            (true, true)
        } else {
            (false, false)
        }
    }

    let (mut a, mut b) = (Some(initial_a), Some(initial_b));

    let mut fixed_smudge = false;
    loop {
        match (a, b) {
            (None, None) => unreachable!(),
            (Some(_a), Some(_b)) => {
                match check_reflection_smudged(group, (_a, _b), &get_row_or_col) {
                    (true, true) => {
                        if fixed_smudge {
                            return None;
                        } else {
                            fixed_smudge = true;
                        }
                    }
                    (true, false) => {}
                    (false, false) => {
                        return None;
                    }
                    (false, true) => unreachable!(),
                }

                (a, b) = move_away_indeces((_a, _b), 1, upper_limit(group));
            }
            (Some(_), None) | (None, Some(_)) => {
                return if fixed_smudge {
                    Some(initial_a + 1)
                } else {
                    None
                };
            }
        }
    }
}

fn calc_group_smudged(group: Group) -> usize {
    let col_indeces = generate_initital_column_indeces(&group);

    let col_reflection = col_indeces
        .into_iter()
        .find_map(|(l, r)| check_full_col_reflection_smudged(&group, (l, r)));

    let row_indeces = generate_initial_row_indeces(&group);

    let row_reflection = row_indeces
        .into_iter()
        .find_map(|(l, r)| check_full_row_reflection_smudged(&group, (l, r)));

    col_reflection.unwrap_or(0) + row_reflection.unwrap_or(0) * 100
}

fn generate_initital_column_indeces(group: &Group) -> Vec<AB> {
    generate_initial_indeces(width(group))
}

fn generate_initial_row_indeces(group: &Group) -> Vec<AB> {
    generate_initial_indeces(height(group))
}

fn generate_initial_indeces(count: usize) -> Vec<AB> {
    (0..count)
        .collect::<Vec<_>>()
        .as_slice()
        .windows(2)
        .map(|window| {
            let (l, r) = (window[0], window[1]);
            (l, r)
        })
        .collect()
}

fn move_away_indeces((a, b): AB, c: usize, upper_limit: usize) -> (Option<usize>, Option<usize>) {
    (
        if c > a { None } else { Some(a - c) },
        if c + b >= upper_limit {
            None
        } else {
            Some(b + c)
        },
    )
}

fn width(group: &Group) -> usize {
    group[0].len()
}

fn height(group: &Group) -> usize {
    group.len()
}

fn get_column(group: &Group, col: usize) -> Vec<Cell> {
    group.into_iter().map(|row| row[col]).collect()
}

fn get_row(group: &Group, row: usize) -> Vec<Cell> {
    group[row].clone()
}

fn parse_group(value: &str) -> Group {
    value
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Cell::Rock,
                    '.' => Cell::Ash,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn parse_input() -> Input {
    let input = std::fs::read_to_string("../inputs/13.txt").unwrap();
    input.split("\n\n").map(parse_group).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_group_1() -> &'static str {
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
    }

    fn test_group_2() -> &'static str {
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    }

    #[test]
    fn test_calc_test_group_1_smudged() {
        let group = parse_group(test_group_1());

        let row_indeces = generate_initial_row_indeces(&group);

        let r = row_indeces
            .into_iter()
            .find_map(|(l, r)| check_full_row_reflection_smudged(&group, (l, r)));
        assert_eq!(r, Some(3));
    }

    #[test]
    fn test_calc_test_group_2_smudged() {
        let group = parse_group(test_group_2());

        let row_indeces = generate_initial_row_indeces(&group);

        let r = row_indeces
            .into_iter()
            .find_map(|(l, r)| check_full_row_reflection_smudged(&group, (l, r)));
        assert_eq!(r, Some(1));
    }

    #[test]
    fn test_calc_test_group_1() {
        let group = parse_group(test_group_1());

        let col_indeces = generate_initital_column_indeces(&group);

        let r = col_indeces
            .into_iter()
            .find_map(|(l, r)| check_full_col_reflection(&group, (l, r)));

        assert_eq!(r, Some(5));
    }

    #[test]
    fn test_calc_test_group_2() {
        let group = parse_group(test_group_2());

        let row_indeces = generate_initial_row_indeces(&group);

        let r = row_indeces
            .into_iter()
            .find_map(|(l, r)| check_full_row_reflection(&group, (l, r)));

        assert_eq!(r, Some(4));
    }
}

#[aoc::main(13)]
fn main(input: &str) -> (usize, usize) {
    //let input = std::fs::read_to_string("../inputs/13.test").unwrap();
    let patterns = input.split("\n\n").collect::<Vec<_>>();
    let mut total = 0;
    for p in &patterns {
        println!("{}", p);
        let grid_len = p.lines().count();
        let cols = p.lines().next().unwrap().len();
        let mut grid = vec![vec![' '; cols]; grid_len];
        for (i, line) in p.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                grid[i][j] = c;
            }
        }

        let factor = find_reflection_point(&grid);
        println!("Factor: {}", factor);
        total += factor;
    }

    println!("Total: {}", total);

    let real = parse_input()
        .into_iter()
        .map(calc_group_smudged)
        .sum::<usize>();
    println!("Real: {}", real);
    (0, 0)
}
