use hashbrown::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn in_bounds(self, max: Point) -> bool {
        self.x >= 0 && self.x < max.x && self.y >= 0 && self.y < max.y
    }
}

fn solve((grid_points, max): &(HashMap<u8, Vec<Point>>, Point)) -> usize {
    let mut grid = vec![vec![false; max.y as usize]; max.x as usize];

    for points in grid_points.values() {
        for &a in points {
            for &b in points.iter().filter(|&&c2| a != c2) {
                let diff = a.sub(b);

                // add extrapolated nodes
                let p1 = a.add(diff);
                let p2 = b.sub(diff);

                if p1.in_bounds(*max) {
                    grid[p1.x as usize][p1.y as usize] = true;
                }
                if p2.in_bounds(*max) {
                    grid[p2.x as usize][p2.y as usize] = true;
                }
            }
        }
    }

    grid.iter().flatten().filter(|&&v| v).count()
}

fn solve2((grid_points, max): &(HashMap<u8, Vec<Point>>, Point)) -> usize {
    let mut grid = vec![vec![false; max.y as usize]; max.x as usize];

    for points in grid_points.values() {
        for &a in points {
            for &b in points.iter().filter(|&&c2| a != c2) {
                let diff = a.sub(b);

                let mut tmp = a;
                while tmp.in_bounds(*max) {
                    grid[tmp.x as usize][tmp.y as usize] = true;
                    tmp.x += diff.x;
                    tmp.y += diff.y;
                }

                let mut tmp = b;
                while tmp.in_bounds(*max) {
                    grid[tmp.x as usize][tmp.y as usize] = true;
                    tmp.x -= diff.x;
                    tmp.y -= diff.y;
                }
            }
        }
    }

    grid.iter().flatten().filter(|&&v| v).count()
}

fn parse(input: &str) -> (HashMap<u8, Vec<Point>>, Point) {
    let mut antennas = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        max_y = y as isize + 1;
        for (x, ch) in line.bytes().enumerate() {
            max_x = max_x.max(x as isize + 1);
            if ch != b'.' {
                antennas
                    .entry(ch)
                    .and_modify(|v: &mut Vec<Point>| v.push(Point::new(x as isize, y as isize)))
                    .or_insert(vec![Point::new(x as isize, y as isize)]);
            }
        }
    }

    (antennas, Point::new(max_x, max_y))
}

#[aoc::main(08)]
fn main(input: &str) -> (usize, usize) {
    let data = parse(input);

    let p1 = solve(&data);
    let p2 = solve2(&data);

    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn test_p1() {
        assert_eq!(solve(&parse(EXAMPLE)), 14);
    }

    #[test]
    fn test_p2() {
        assert_eq!(solve2(&parse(EXAMPLE)), 34);
    }
}
