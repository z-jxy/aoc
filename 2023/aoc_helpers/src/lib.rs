use std::fmt::Display;

mod grid;

pub use grid::Grid;

#[cfg(test)]
mod tests {
    use crate::grid::Grid;

    use super::*;
    const DATA: &str = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    #[test]
    fn it_works() {
        let example = DATA
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let data: Grid<usize> = Grid::from(DATA);
        println!("{}", data);
        //assert_eq!(result, 4);
    }
}
