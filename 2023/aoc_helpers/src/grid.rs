use std::fmt::Display;

pub struct Grid<T> {
    width: usize,
    height: usize,
    grid: Vec<Vec<T>>,
}

impl<T> From<T> for Grid<char>
where
    T: Into<String> + AsRef<str>,
{
    fn from(s: T) -> Self {
        let grid = s
            .into()
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let width = grid[0].len();
        let height = grid.len();
        Self {
            width,
            height,
            grid,
        }
    }
}

impl<T> From<T> for Grid<usize>
where
    T: Into<String> + AsRef<str>,
{
    fn from(s: T) -> Self {
        let grid = s
            .into()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        let width = grid[0].len();
        let height = grid.len();
        Self {
            width,
            height,
            grid,
        }
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid {
            for c in line {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
