const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

const P1_DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1), // up-left
    (-1, 0),  // up
    (-1, 1),  // up-right
    (0, 1),   // right
    (1, 1),   // down-right
    (1, 0),   // down
    (1, -1),  // down-left
    (0, -1),  // left
];

const P2_DIRECTIONS: [(i32, i32); 4] = [
    (-1, -1), // up-left
    (1, 1),   // down-right
    (-1, 1),  // up-right
    (1, -1),  // down-left
];

pub fn find_xmas(grid: &[Vec<char>], r: usize, c: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    assert!(grid[r][c] == XMAS[0]);

    let mut count = 0;
    for (dr, dc) in P1_DIRECTIONS.iter() {
        let mut xmas_idx = 1;
        let mut nr = r as i32 + dr;
        let mut nc = c as i32 + dc;

        while nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
            if grid[nr as usize][nc as usize] != XMAS[xmas_idx] {
                break;
            }
            xmas_idx += 1;

            if xmas_idx == XMAS.len() {
                count += 1;
                break;
            }

            nr += dr;
            nc += dc;
        }
    }

    count
}

fn check_mas_pair(
    grid: &[Vec<char>],
    r: usize,
    c: usize,
    rows: usize,
    cols: usize,
    dir1: (i32, i32),
    dir2: (i32, i32),
) -> bool {
    let (dr1, dc1) = (r as i32 + dir1.0, c as i32 + dir1.1);
    let (dr2, dc2) = (r as i32 + dir2.0, c as i32 + dir2.1);

    // Bounds checking
    if dr1 < 0 || dr1 >= rows as i32 || dc1 < 0 || dc1 >= cols as i32 {
        return false;
    }
    if dr2 < 0 || dr2 >= rows as i32 || dc2 < 0 || dc2 >= cols as i32 {
        return false;
    }

    // Character matching
    let char1 = grid[dr1 as usize][dc1 as usize];
    let char2 = grid[dr2 as usize][dc2 as usize];
    (char1 == 'M' && char2 == 'S') || (char1 == 'S' && char2 == 'M')
}

pub fn find_x_mas(grid: &[Vec<char>], r: usize, c: usize) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    let [d_tl, d_br, d_tr, d_bl] = P2_DIRECTIONS;

    let x1 = check_mas_pair(grid, r, c, rows, cols, d_tl, d_br);
    let x2 = check_mas_pair(grid, r, c, rows, cols, d_tr, d_bl);

    x1 && x2
}

pub fn solve(grid: &[Vec<char>]) -> (usize, usize) {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut p1 = 0;
    let mut p2 = 0;

    for r in 0..rows {
        for c in 0..cols {
            match grid[r][c] {
                'X' => p1 += find_xmas(grid, r, c),
                'A' if find_x_mas(grid, r, c) => p2 += 1,
                _ => {}
            }
        }
    }

    (p1, p2)
}

#[aoc::main(04)]
fn main(input: &str) -> (usize, usize) {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (p1, p2) = solve(&grid);

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn p1() {
        let mut grid = EXAMPLE
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let (p1, _) = solve(&mut grid);
        assert_eq!(p1, 18);
    }

    #[test]
    fn p2() {
        let mut grid = EXAMPLE
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let (_, p2) = solve(&mut grid);
        assert_eq!(p2, 9);
    }
}
