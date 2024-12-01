use std::collections::BTreeMap;

fn parse_solar_system(input: &str) -> Vec<Vec<char>> {
    let mut solar_system = Vec::new();

    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        let has_galaxy = row.iter().any(|&c| c == '#');
        solar_system.push(row.clone());
        if !has_galaxy {
            // Double the row if no galaxy is present
            solar_system.push(row);
        }
    }
    // Step 2: Process columns
    let num_columns = solar_system[0].len();
    let mut columns_to_double = vec![true; num_columns];

    // Identify columns that need to be doubled
    for row in &solar_system {
        for (i, &c) in row.iter().enumerate() {
            if c == '#' {
                columns_to_double[i] = false;
            }
        }
    }

    // Create the new solar system with adjusted columns
    let mut expanded_solar_system = Vec::new();
    for row in solar_system {
        let mut new_row = Vec::new();
        for (i, &c) in row.iter().enumerate() {
            new_row.push(c);
            if columns_to_double[i] {
                // Double the column by adding the character again
                new_row.push(c);
            }
        }
        expanded_solar_system.push(new_row);
    }

    expanded_solar_system
}

fn parse_base_solar_system(input: &str) -> Vec<Vec<char>> {
    let mut solar_system = Vec::new();

    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        let has_galaxy = row.iter().any(|&c| c == '#');
        solar_system.push(row.clone());
        if !has_galaxy {
            // Double the row if no galaxy is present
            solar_system.push(row);
        }
    }
    // Step 2: Process columns
    let num_columns = solar_system[0].len();
    let mut columns_to_double = vec![true; num_columns];

    // Identify columns that need to be doubled
    for row in &solar_system {
        for (i, &c) in row.iter().enumerate() {
            if c == '#' {
                columns_to_double[i] = false;
            }
        }
    }

    // Create the new solar system with adjusted columns
    let mut expanded_solar_system = Vec::new();
    for row in solar_system {
        let mut new_row = Vec::new();
        for (i, &c) in row.iter().enumerate() {
            new_row.push(c);
            if columns_to_double[i] {
                // Double the column by adding the character again
                new_row.push(c);
            }
        }
        expanded_solar_system.push(new_row);
    }

    expanded_solar_system
}

fn expand_solar_system(input: &str, expansion_factor: usize) -> Vec<Vec<char>> {
    let mut solar_system = Vec::new();

    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        let has_galaxy = row.iter().any(|&c| c == '#');
        solar_system.push(row.clone());
        if !has_galaxy {
            // Double the row if no galaxy is present
            for _ in 0..expansion_factor - 1 {
                solar_system.push(row.clone());
            }
        }
    }
    // Step 2: Process columns
    let num_columns = solar_system[0].len();
    let mut columns_to_double = vec![true; num_columns];

    // Identify columns that need to be doubled
    for row in &solar_system {
        for (i, &c) in row.iter().enumerate() {
            if c == '#' {
                columns_to_double[i] = false;
            }
        }
    }

    // Create the new solar system with adjusted columns
    let mut expanded_solar_system = Vec::new();
    for row in solar_system {
        let mut new_row = Vec::new();
        for (i, &c) in row.iter().enumerate() {
            new_row.push(c);
            if columns_to_double[i] {
                // Double the column by adding the character again
                for _ in 0..expansion_factor - 1 {
                    new_row.push(c);
                }
            }
        }
        expanded_solar_system.push(new_row);
    }

    expanded_solar_system
}

fn get_solar_system(input: &str, expansion_factor: usize) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    let mut total_empty_rows = 0;
    let mut empty_cols_count = vec![0; input.lines().next().unwrap_or("").len()];
    let mut current_empty_cols = vec![0; empty_cols_count.len()];

    for (i, line) in input.lines().enumerate() {
        let row: Vec<char> = line.chars().collect();
        let mut is_row_empty = true;

        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                galaxies.push((i, j, total_empty_rows, current_empty_cols[j]));
                is_row_empty = false;
                current_empty_cols[j] = 0; // Reset the column count for columns with galaxies
            } else {
                current_empty_cols[j] += 1;
            }
        }

        if is_row_empty {
            total_empty_rows += 1;
        } else {
            for j in 0..empty_cols_count.len() {
                empty_cols_count[j] = std::cmp::max(empty_cols_count[j], current_empty_cols[j]);
            }
        }
    }
    let galx = galaxies
        .into_iter()
        .map(|(row, col, empty_rows_up_to, empty_cols_up_to)| {
            let adjusted_row = row + empty_rows_up_to * (expansion_factor - 1);
            let adjusted_col = col + empty_cols_up_to * (expansion_factor - 1);
            (adjusted_row, adjusted_col)
        })
        .collect();
    galx
}

mod tests {
    use super::*;
    #[test]
    fn distances() {
        let input = std::fs::read_to_string("../inputs/11.test").unwrap();
        let mut galaxies = BTreeMap::new();

        let solar_system = parse_solar_system(&input);

        for (i, row) in solar_system.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == '#' {
                    galaxies.insert(galaxies.len() + 1, (i, j));
                }

                print!("{}", c);
            }

            println!();
        }

        // find the total number of pairs of galaxies
        let pairs = galaxies.len() * (galaxies.len() - 1) / 2;

        println!("pairs: {}", pairs);

        let start = galaxies.get(&5).unwrap();
        let end = galaxies.get(&9).unwrap();

        let test_pairs = [
            (galaxies.get(&5).unwrap(), galaxies.get(&9).unwrap()),
            (galaxies.get(&1).unwrap(), galaxies.get(&7).unwrap()),
            (galaxies.get(&3).unwrap(), galaxies.get(&6).unwrap()),
            (galaxies.get(&8).unwrap(), galaxies.get(&9).unwrap()),
        ];
        for p in test_pairs.iter() {
            let distance = (end.0 as isize - start.0 as isize).abs()
                + (end.1 as isize - start.1 as isize).abs();
            println!("distance: {}", distance);
        }
    }
}

fn p1(input: &str) -> usize {
    let mut galaxies = BTreeMap::new();

    let solar_system = parse_solar_system(&input);

    for (i, row) in solar_system.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                galaxies.insert(galaxies.len() + 1, (i, j));
            }

            print!("{}", c);
        }

        println!();
    }

    // find the total number of pairs of galaxies
    let pairs = galaxies.len() * (galaxies.len() - 1) / 2;

    println!("pairs: {}", pairs);

    // for each of the pairs, find the shortest path between the galaxies
    let mut total_sum = 0;
    for (i, start) in galaxies.iter() {
        for (j, end) in galaxies.range((i + 1)..) {
            let distance = (end.0 as isize - start.0 as isize).abs()
                + (end.1 as isize - start.1 as isize).abs();
            println!("distance: {}", distance);

            total_sum += distance as usize;
        }
    }

    println!("total sum: {}", total_sum);

    total_sum
}

fn calculate_distance(
    start: (usize, usize),
    end: (usize, usize),
    expansion_factor: isize,
) -> isize {
    let row_distance = if start.0 == end.0 {
        0
    } else {
        (end.0 as isize - start.0 as isize).abs() * expansion_factor
    };

    let col_distance = if start.1 == end.1 {
        0
    } else {
        (end.1 as isize - start.1 as isize).abs() * expansion_factor
    };

    row_distance + col_distance
}

const GRID_SIZE: usize = 140;

#[aoc::main(11)]
fn main(input: &str) -> (usize, usize) {
    let input = input.trim(); // println!("{input}");

    // Count the nodes in each row, column, and in total.
    let mut rows = [0; GRID_SIZE];
    let mut cols = [0; GRID_SIZE];
    let mut count = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                // println!("Galaxy at {row} {col}");
                rows[row] += 1;
                cols[col] += 1;
                count += 1;
            }
        }
    }
    // println!("rows: #{rows:?}");
    // println!("cols: #{cols:?}");

    println!("Part 1: {}", calculate(&rows, &cols, count, 2));
    println!("Part 2: {}", calculate(&rows, &cols, count, 1_000_000));

    (0, 0)
}

fn calculate(
    rows: &[usize; GRID_SIZE],
    cols: &[usize; GRID_SIZE],
    count: usize,
    scale: usize,
) -> usize {
    let mut total = 0;
    let mut running_count = (0, 0);
    for i in 0..GRID_SIZE {
        running_count.0 += rows[i];
        running_count.1 += cols[i];
        total += running_count.0 * (count - running_count.0) * if rows[i] == 0 { scale } else { 1 };
        total += running_count.1 * (count - running_count.1) * if cols[i] == 0 { scale } else { 1 };
    }
    total
}
