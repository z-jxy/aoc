#[aoc::main(22)]
fn main(input: &str) -> (usize, usize) {
    let input = std::fs::read_to_string("../inputs/22.test").unwrap();
    let bricks = input
        .lines()
        .map(|line| {
            let mut parts = line.split("~");

            let start = parts
                .next()
                .unwrap()
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let end = parts
                .next()
                .unwrap()
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            (start, end)
        })
        .collect::<Vec<_>>();

    println!("{:?}", bricks);

    (0, 0)
}
