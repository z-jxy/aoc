use hashbrown::HashSet;
use num::Integer;

#[aoc::main(21)]
pub fn main(input: &str) -> (usize, usize) {
    let mut input = std::fs::read_to_string("../inputs/21.test").unwrap();
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = grid
        .iter()
        .enumerate()
        .find(|(_, row)| row.iter().position(|&c| c == 'S').is_some())
        .map(|(y, row)| (y, row.iter().position(|&c| c == 'S').unwrap()))
        .unwrap();

    println!("{:?}", grid[5][5]);

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut plots_reached = HashSet::new();
    let mut steps = 0;

    let mut queue = vec![start];
    const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while steps != 6 {
        if let Some((y, x)) = queue.pop() {
            if visited[y][x] {
                continue;
            }
            visited[y][x] = true;

            if grid[y][x] == 'E' {
                println!("found exit at {}, {}", y, x);
                break;
            }

            for d in DIRECTIONS.iter() {
                let (dr, dc) = d;
                let (nr, nc) = (y as isize + dr, x as isize + dc);
                if nr < 0 || nr >= grid.len() as isize || nc < 0 || nc >= grid[0].len() as isize {
                    continue;
                }

                let (nr, nc) = (nr as usize, nc as usize);

                match grid[nr][nc] {
                    '#' => {
                        //visited[y][x] = true;
                        //plots_reached.insert((y, x));
                    }
                    '.' => {
                        queue.push((nr, nc));
                        plots_reached.insert((nr, nc));
                    }
                    _ => {}
                }
            }

            steps += 1;
        }

        // if y < grid.len() - 1 && grid[y + 1][x] != '#' {
        //     queue.push((y + 1, x));
        // }
        // if x > 0 && grid[y][x - 1] != '#' {
        //     queue.push((y, x - 1));
        // }
        // if x < grid[y].len() - 1 && grid[y][x + 1] != '#' {
        //     queue.push((y, x + 1));
        // }
    }

    //println!("{:?}", plots_reached);
    //println!("{:?}", visited);

    let mut debug = grid.clone();
    for (y, row) in debug.iter_mut().enumerate() {
        for (x, c) in row.iter_mut().enumerate() {
            if visited[y][x] {
                *c = 'O';
            }
        }
    }

    for row in debug {
        for c in row {
            print!("{}", c);
        }
        println!();
    }

    (0, 0)
}
