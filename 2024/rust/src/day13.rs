use aoc_runner_derive::{aoc, aoc_generator};

const COST_A: u8 = 3;
const COST_B: u8 = 1;

struct Cord {
    x: usize,
    y: usize,
}

struct Machine {
    a: Cord,
    b: Cord,
    prize: Cord,
}

fn parse_button(line: &str) -> Cord {
    let button: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    let x_slice = button[2][2..button[2].len() - 1].to_string();
    let y_slice = button[3][2..].to_string();

    Cord {
        x: x_slice.parse().unwrap(),
        y: y_slice.parse().unwrap(),
    }
}

fn parse_prize(line: &str) -> Cord {
    let button: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    let x_slice = button[1][2..button[1].len() - 1].to_string();
    let y_slice = button[2][2..].to_string();

    Cord {
        x: x_slice.parse().unwrap(),
        y: y_slice.parse().unwrap(),
    }
}

#[aoc_generator(day13, part1)]
fn parse(input: &str) -> Vec<Machine> {
    let mut blocks = input.split("\n\n");

    let mut machines = vec![];

    while let Some(block) = blocks.next() {
        let mut lines = block.lines();
        let a_button = parse_button(lines.next().unwrap());
        let b_button = parse_button(lines.next().unwrap());
        let prize = parse_prize(lines.next().unwrap());

        let machine = Machine {
            a: a_button,
            b: b_button,
            prize: prize,
        };
        machines.push(machine);
    }

    machines
}

#[aoc_generator(day13, part2)]
fn parse2(input: &str) -> Vec<Machine> {
    let mut blocks = input.split("\n\n");

    let mut machines = vec![];
    while let Some(block) = blocks.next() {
        let mut lines = block.lines();

        let a_button = parse_button(lines.next().unwrap());
        let b_button = parse_button(lines.next().unwrap());
        let prize = parse_prize(lines.next().unwrap());

        let machine = Machine {
            a: a_button,
            b: b_button,
            prize: Cord {
                x: prize.x + 10000000000000,
                y: prize.y + 10000000000000,
            },
        };
        machines.push(machine);
    }

    machines
}

#[aoc(day13, part1, bruteforce)]
fn part1(machines: &[Machine]) -> usize {
    let mut min_tokens_all_prize = 0;
    const MAX_A_PRESSES: usize = 100;
    const MAX_B_PRESSES: usize = 100;
    for machine in machines.iter() {
        let prize = &machine.prize;
        let a_move = (machine.a.x, machine.a.y);
        let b_move = (machine.b.x, machine.b.y);

        // brute-force approach to find the minimum token cost

        let mut current_min_tokens = usize::MAX;

        for a_presses in 0..=MAX_A_PRESSES {
            for b_presses in 0..=MAX_B_PRESSES {
                // calculate x and y cords after button presses
                let x_coord = a_presses * a_move.0 + b_presses * b_move.0;
                let y_coord = a_presses * a_move.1 + b_presses * b_move.1;

                // check if this combination reaches the prize exactly
                if x_coord == prize.x && y_coord == prize.y {
                    let tokens = a_presses * 3 + b_presses;
                    current_min_tokens = current_min_tokens.min(tokens);
                }
            }
        }

        if current_min_tokens == usize::MAX {
            // no valid combination found
            continue;
        }

        min_tokens_all_prize += current_min_tokens;
    }

    min_tokens_all_prize
}

#[aoc(day13, part1, optimized)]
fn part1_opt(machines: &[Machine]) -> usize {
    machines
        .iter()
        .filter_map(|m| calc_token_cost(m))
        .sum::<usize>()
}

#[aoc(day13, part2)]
fn part2(machines: &[Machine]) -> usize {
    machines
        .iter()
        .filter_map(|m| calc_token_cost(m))
        .sum::<usize>()
}

fn calc_token_cost(machine: &Machine) -> Option<usize> {
    let prize = &machine.prize;
    let (a_x, a_y) = (machine.a.x, machine.a.y);
    let (b_x, b_y) = (machine.b.x, machine.b.y);

    // solve the system of equations:
    // a_x * A + b_x * B = prize.x
    // a_y * A + b_y * B = prize.y
    //
    // using matrix form and Cramer's rule:
    // D = a_x*b_y - b_x*a_y
    // A = (b_y*prize.x - b_x*prize.y) / D
    // B = (-a_y*prize.x + a_x*prize.y) / D

    let d = (a_x as i128) * (b_y as i128) - (b_x as i128) * (a_y as i128);

    // compute potential solutions
    let numerator_a = (b_y as i128) * (prize.x as i128) - (b_x as i128) * (prize.y as i128);
    let numerator_b = -(a_y as i128) * (prize.x as i128) + (a_x as i128) * (prize.y as i128);

    // check if both numerators are divisible by d
    if numerator_a % d != 0 || numerator_b % d != 0 {
        // no solution
        return None;
    }

    let a_presses = (numerator_a / d) as isize;
    let b_presses = (numerator_b / d) as isize;

    // calculate the token cost
    let tokens =
        (a_presses as usize) * (COST_A as usize) + (b_presses as usize) * (COST_B as usize);

    Some(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 480);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2(EXAMPLE)), 875318608908);
    }
}
