use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

#[derive(Debug, Clone)]
struct Robot {
    pos: (usize, usize),
    vel: (i32, i32),
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (p_part, v_part) = line.split_once(' ').unwrap();

            let p = p_part[2..].split_once(',').unwrap();
            let v = v_part[2..].split_once(',').unwrap();

            let p_x = p.0.parse::<usize>().unwrap();
            let p_y = p.1.parse::<usize>().unwrap();
            let v_x = v.0.parse::<i32>().unwrap();
            let v_y = v.1.parse::<i32>().unwrap();

            Robot {
                pos: (p_x, p_y),
                vel: (v_x, v_y),
            }
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(robots: &[Robot]) -> usize {
    let mut robots = robots.to_vec();

    const SECONDS: u8 = 100;
    let mut quadrant_counts = [0; 4];

    for bot in robots.iter_mut() {
        bot.pos.1 = ((bot.pos.1 as i32 + bot.vel.1 * SECONDS as i32) % HEIGHT as i32
            + HEIGHT as i32) as usize
            % HEIGHT;
        bot.pos.0 = ((bot.pos.0 as i32 + bot.vel.0 * SECONDS as i32) % WIDTH as i32 + WIDTH as i32)
            as usize
            % WIDTH;

        match (bot.pos.1, bot.pos.0) {
            (r, c) if r < HEIGHT / 2 && c < WIDTH / 2 => quadrant_counts[0] += 1,
            (r, c) if r < HEIGHT / 2 && c > WIDTH / 2 => quadrant_counts[1] += 1,
            (r, c) if r > HEIGHT / 2 && c < WIDTH / 2 => quadrant_counts[2] += 1,
            (r, c) if r > HEIGHT / 2 && c > WIDTH / 2 => quadrant_counts[3] += 1,
            _ => {}
        }
    }

    quadrant_counts.iter().product()
}

#[aoc(day14, part2, _)]
fn part2(robots: &[Robot]) -> usize {
    let mut robots = robots.to_vec();

    let mut positions: HashSet<(usize, usize)> = HashSet::with_capacity(robots.len());
    let mut elapsed = 0;
    loop {
        elapsed += 1;

        for bot in robots.iter_mut() {
            bot.pos.1 =
                ((bot.pos.1 as i32 + bot.vel.1) % HEIGHT as i32 + HEIGHT as i32) as usize % HEIGHT;
            bot.pos.0 =
                ((bot.pos.0 as i32 + bot.vel.0) % WIDTH as i32 + WIDTH as i32) as usize % WIDTH;

            positions.insert(bot.pos);
        }

        if positions.len() == robots.len() {
            break;
        }

        positions.clear();
    }

    elapsed
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 21);
    }
}
