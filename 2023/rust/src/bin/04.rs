use std::collections::HashSet;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::digit0,
    multi::separated_list1,
    sequence::{terminated, tuple},
    Finish, IResult,
};

fn parse_card_nums(input: &str) -> IResult<&str, HashSet<usize>> {
    separated_list1(tag(" "), digit0)(input).map(|(input, res)| {
        let res = res
            .iter()
            .filter_map(|s| {
                if s.is_empty() {
                    return None;
                }
                Some(s.parse::<usize>().unwrap())
            })
            .collect::<HashSet<usize>>();

        (input, res)
    })
}

fn parse_scratchcard(
    input: &str,
) -> Result<(HashSet<usize>, HashSet<usize>), nom::error::Error<&str>> {
    tuple((
        take_while1(|c: char| c != ':'),
        tag(": "),
        terminated(parse_card_nums, tag("|")),
        parse_card_nums,
    ))(input)
    .map(|(_, (_, _, winners, nums))| ((winners, nums)))
    .finish()
}

#[aoc::main(04)]
fn main(input: &str) -> (usize, usize) {
    let mut played = vec![0; input.lines().count()];

    let points = input.lines().enumerate().filter_map(|(i, line)| {
        let (winners, nums) = parse_scratchcard(line).unwrap();
        played[i] += 1;

        let mut points = None;

        let winning: Vec<&usize> = winners
            .intersection(&nums)
            .map(|winner| {
                // solve part 1
                if let Some(matches) = points {
                    points = Some(matches * 2);
                } else {
                    points = Some(1)
                }

                winner
            })
            .collect();

        // solve part 2 by processing the copies
        for w in 0..winning.len() {
            played[i + w + 1] += played[i]
        }

        points
    });

    (points.sum::<usize>(), played.iter().sum::<usize>())
}
