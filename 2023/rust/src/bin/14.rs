#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Position {
    row: usize,
    col: usize,
}

impl Direction {
    fn shift(&self, grid: &[Vec<u8>], pos: &Position) -> Position {
        let Position { row, col } = *pos;
        match self {
            Direction::North => {
                let mut irow = pos.row;

                while irow > 0 && grid[irow - 1][col] == b'.' {
                    irow -= 1;
                }

                Position { row: irow, col }
            }
            Direction::West => {
                let mut icol = pos.col;
                while icol > 0 && grid[row][icol - 1] == b'.' {
                    icol -= 1;
                }
                Position { row, col: icol }
            }
            Direction::South => {
                let mut irow = pos.row;
                while irow < grid.len() - 1 && grid[irow + 1][col] == b'.' {
                    irow += 1;
                }
                Position { row: irow, col }
            }
            Direction::East => {
                let mut icol = pos.col;

                while icol < grid[row].len() - 1 && grid[row][icol + 1] == b'.' {
                    icol += 1;
                }
                Position { row, col: icol }
            }
        }
    }
}

fn tilt(d: &Direction, tilted: &mut [Vec<u8>], pos: Position) {
    if tilted[pos.row][pos.col] == b'O' {
        let shifted_pos = d.shift(&tilted, &pos);
        tilted[shifted_pos.row][shifted_pos.col] = b'O';
        if shifted_pos != pos {
            tilted[pos.row][pos.col] = b'.';
        }
    }
}

fn simulate_cycle(mut tilted: &mut [Vec<u8>]) {
    const DIRECTIONS: [Direction; 4] = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];
    for d in DIRECTIONS.iter() {
        match d {
            Direction::East => {
                for row in 0..tilted.len() {
                    for col in (0..tilted[row].len()).rev() {
                        tilt(&d, tilted, Position { row, col });
                    }
                }
            }
            Direction::South => {
                for row in (0..tilted.len()).rev() {
                    for col in 0..tilted[row].len() {
                        tilt(&d, &mut tilted, Position { row, col });
                    }
                }
            }
            _ => {
                (0..tilted.len()).for_each(|row| {
                    (0..tilted[row].len()).for_each(|col| {
                        tilt(&d, &mut tilted, Position { row, col });
                    })
                });
            }
        }
    }

    //tilted
}

fn cycle(mut grid: &mut [Vec<u8>], cycles: usize) {
    let mut seen: Vec<Vec<Vec<u8>>> = Vec::new();
    loop {
        simulate_cycle(&mut grid);
        if let Some(idx) = seen.iter().position(|g| g == &grid) {
            let first_cycle_of_pattern = idx + 1;
            let cycle_length = (seen.len() + 1) - first_cycle_of_pattern;
            let remaining_cycles = (cycles - first_cycle_of_pattern) % cycle_length;

            let res = &seen[idx + remaining_cycles];

            grid.as_mut()
                .iter_mut()
                .zip(res.iter())
                .for_each(|(row, x)| {
                    row.iter_mut().zip(x.iter()).for_each(|(c, x)| {
                        *c = *x;
                    })
                });

            return;
        }
        seen.push(grid.to_owned());
    }
}

fn p1(lines: &Vec<Vec<u8>>) -> usize {
    let mut tilted = vec![vec![b'.'; lines[0].len()]; lines.len()];
    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            let symbol = lines[row][col];
            if symbol != b'O' {
                tilted[row][col] = symbol;
                continue;
            }
            let mut irow = row;
            while irow > 0 && tilted[irow - 1][col] == b'.' {
                irow -= 1;
            }
            tilted[irow][col] = b'O';
        }
    }

    let mut load_m = tilted.len();
    let sum = tilted
        .iter()
        .map(|row| {
            let rocks = row.iter().filter(|&&c| c == b'O').count();
            let res = rocks * load_m;
            load_m -= 1;
            res
        })
        .sum::<usize>();
    sum
}

fn p2(mut grid: &mut [Vec<u8>]) -> usize {
    cycle(&mut grid, 1_000_000_000);
    let mut load_m = grid.len();
    let sum = grid
        .iter()
        .map(|row| {
            let rocks = row.iter().filter(|&&c| c == b'O').count();
            let res = rocks * load_m;
            load_m -= 1;
            res
        })
        .sum::<usize>();
    sum
}

#[aoc::main(14)]
fn main(input: &str) -> (usize, usize) {
    let mut lines = input
        .lines()
        .map(|row| row.chars().map(|c| c as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let p1 = p1(&lines);
    let p2 = p2(&mut lines);

    assert!(p2 == 93102 || p2 == 64);
    (p1, p2)
}
