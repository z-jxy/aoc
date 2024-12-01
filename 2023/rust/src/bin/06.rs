fn solve(time: usize, distance: usize) -> usize {
    let b = time as f64;
    let c = distance as f64;
    let disc = (b * b - 4.0 * c).sqrt();

    // Calculate the minimum and maximum time bounds
    let min = ((b - disc) / 2.0 + 1.0).floor();
    let max = ((b + disc) / 2.0 - 1.0).ceil();

    (max - min + 1.0) as usize
}

fn parse_line_to_vec(line: Option<&str>) -> Vec<usize> {
    line.unwrap() // Unwrap the Option<&str>
        .split(":") // Split the line at ':'
        .nth(1) // Take the second part (after the ':')
        .unwrap() // Unwrap the Option<&str>
        .split_whitespace() // Split the line on whitespace
        .map(|v| v.parse().unwrap()) // Parse each value to usize
        .collect() // Collect values into a Vec<usize>
}

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    let times = parse_line_to_vec(lines.next());
    let distances = parse_line_to_vec(lines.next());

    let p1 = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| solve(*time, *distance))
        .product::<usize>();

    let time = format!(
        "{}",
        times
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
            .join(""),
    )
    .parse::<usize>()
    .unwrap();

    let distance = format!(
        "{}",
        distances
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
            .join(""),
    )
    .parse::<usize>()
    .unwrap();

    let p2 = solve(time, distance);

    (p1, p2)
}
