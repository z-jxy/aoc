#[inline(always)]
fn set_visited(visited: &mut [u8], r: usize, c: usize, bit: u8, cols: usize) {
    let idx = r * cols + c;
    visited[idx] |= bit;
}

#[inline(always)]
fn is_visited(visited: &[u8], r: usize, c: usize, bit: u8, cols: usize) -> bool {
    let idx = r * cols + c;
    (visited[idx] & bit) != 0
}

#[inline(always)]
fn direction_bit(dr: i32, dc: i32) -> u8 {
    match (dr, dc) {
        (-1, 0) => 0x01, // Up
        (0, 1) => 0x02,  // Right
        (1, 0) => 0x04,  // Down
        (0, -1) => 0x08, // Left
        _ => unreachable!(),
    }
}

fn solve(grid: &mut [Vec<u8>]) -> (usize, usize) {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = vec![vec![false; cols]; rows];

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find_map(|(c, &cell)| if cell == b'^' { Some((r, c)) } else { None })
        })
        .unwrap();

    let (mut r, mut c) = start;

    let (mut dr, mut dc) = (-1, 0);

    let mut potential_positions = Vec::with_capacity(rows * cols);
    loop {
        if !visited[r][c] {
            visited[r][c] = true;
            potential_positions.push((r, c));
        }

        let (nr, nc) = (r as i32 + dr, c as i32 + dc);

        if nr < 0 || nr as usize >= rows || nc < 0 || nc as usize >= cols {
            break;
        }

        if grid[nr as usize][nc as usize] == b'#' {
            (dr, dc) = (dc, -dr);
            continue;
        }

        (r, c) = (nr as usize, nc as usize);
    }

    // potential positions is the solution to p1, excluding the starting position
    let p1 = potential_positions.len(); // save answer for p1
    potential_positions.remove(0); // exclude starting position

    let mut visited_states = vec![0u8; rows * cols];

    let p2 = potential_positions.iter().fold(0, |acc, (or, oc)| {
        let mut looped = 0;

        let original = grid[*or][*oc];

        grid[*or][*oc] = b'#';

        let (mut r, mut c) = start;

        let (mut dr, mut dc) = (-1, 0);

        loop {
            let bit = direction_bit(dr, dc);
            if is_visited(&visited_states, r, c, bit, cols) {
                looped = 1;
                break;
            }
            set_visited(&mut visited_states, r, c, bit, cols);

            let (nr, nc) = (r + dr as usize, c + dc as usize);

            if nr as usize >= rows || nc as usize >= cols {
                break;
            }

            if grid[nr as usize][nc as usize] == b'#' {
                (dr, dc) = (dc, -dr);
                continue;
            }

            (r, c) = (nr as usize, nc as usize);
        }

        grid[*or][*oc] = original; // restore

        visited_states.fill(0); // reset visited states

        acc + looped
    });

    (p1, p2)
}

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    let mut grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    solve(&mut grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_p1() {
        let (p1, _) = solve(
            &mut EXAMPLE
                .lines()
                .map(|line| line.as_bytes().to_vec())
                .collect::<Vec<_>>(),
        );
        assert_eq!(p1, 41);
    }

    #[test]
    fn test_p2() {
        let (_, p2) = solve(
            &mut EXAMPLE
                .lines()
                .map(|line| line.as_bytes().to_vec())
                .collect::<Vec<_>>(),
        );
        assert_eq!(p2, 6);
    }
}
