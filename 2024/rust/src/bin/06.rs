const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[inline(always)]
fn set_visited(visited: &mut [u8], r: usize, c: usize, dir: usize, cols: usize) {
    let idx = (r * cols + c) * 4 + dir;
    visited[idx / 8] |= 1 << (idx % 8);
}

#[inline(always)]
fn is_visited(visited: &[u8], r: usize, c: usize, dir: usize, cols: usize) -> bool {
    let idx = (r * cols + c) * 4 + dir;
    (visited[idx / 8] & (1 << (idx % 8))) != 0
}

fn solve(grid: &mut [Vec<char>]) -> (usize, usize) {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = vec![vec![false; cols]; rows];

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find_map(|(c, &cell)| if cell == '^' { Some((r, c)) } else { None })
        })
        .unwrap();

    let (mut r, mut c) = start;
    let mut dir = 0; // starting direction: up (-1,0) = 0

    loop {
        visited[r][c] = true;

        let nr = r as i32 + DIRECTIONS[dir].0;
        let nc = c as i32 + DIRECTIONS[dir].1;

        if nr < 0 || nr as usize >= rows || nc < 0 || nc as usize >= cols {
            break;
        }

        if grid[nr as usize][nc as usize] == '#' {
            // turn right
            dir = (dir + 1) % 4;
            continue;
        }

        (r, c) = (nr as usize, nc as usize);
    }

    // don't include the starting position
    visited[start.0][start.1] = false;

    let mut potential_positions = vec![];
    for i in 0..visited.len() {
        for j in 0..visited[i].len() {
            if visited[i][j] {
                potential_positions.push((i, j));
            }
        }
    }

    let p1 = potential_positions.len() + 1; // +1 to account for the starting position we set to false

    let size = (rows * cols * 4).div_ceil(8);
    let mut visited_states = vec![0u8; size];
    let p2 = potential_positions.iter().fold(0, |acc, (or, oc)| {
        let mut looped = 0;

        let original = grid[*or][*oc];

        grid[*or][*oc] = '#';

        let (mut r, mut c) = start;
        let mut dir = 0;

        loop {
            if is_visited(&visited_states, r, c, dir, cols) {
                looped = 1;
                break;
            }
            set_visited(&mut visited_states, r, c, dir, cols);

            let nr = r as i32 + DIRECTIONS[dir].0;
            let nc = c as i32 + DIRECTIONS[dir].1;

            if nr < 0 || nr as usize >= rows || nc < 0 || nc as usize >= cols {
                break;
            }

            if grid[nr as usize][nc as usize] == '#' {
                dir = (dir + 1) % 4;
                continue;
            }

            (r, c) = (nr as usize, nc as usize);
        }

        grid[*or][*oc] = original; // restore

        visited_states.iter_mut().for_each(|v| *v = 0); // reset visited states

        acc + looped
    });

    (p1, p2)
}

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
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
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );
        assert_eq!(p1, 41);
    }

    #[test]
    fn test_p2() {
        let (_, p2) = solve(
            &mut EXAMPLE
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );
        assert_eq!(p2, 6);
    }
}
