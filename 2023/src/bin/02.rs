use nom::{
    bytes::complete::{tag, take_while},
    character::complete::digit1,
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

fn game_id(input: &str) -> IResult<&str, u32> {
    tuple((tag("Game "), digit1, tag(": ")))(input)
        .map(|(i, (_, id, _))| (i, id.parse::<u32>().unwrap()))
}

fn parse_round(input: &str) -> IResult<&str, Vec<(u8, &str)>> {
    tuple((separated_list1(
        tag(", "),
        nom::sequence::tuple((
            terminated(digit1, tag(" ")),
            take_while(|c: char| c.is_alphabetic()),
        )),
    ),))(input)
    .map(|(i, (v,))| {
        let mut v2 = Vec::new();
        for (a, b) in v {
            v2.push((a.parse::<u8>().unwrap(), b));
        }
        (i, v2)
    })
}

fn parse_game(input: &str) -> IResult<&str, (u32, Vec<Vec<(u8, &str)>>)> {
    tuple((game_id, separated_list1(tag("; "), parse_round)))(input)
        .map(|(i, (id, v))| (i, (id, v)))
}

#[aoc::main(02)]
fn main(input: &str) -> (usize, usize) {
    const RED_CUBES: u8 = 12;
    const GREEN_CUBES: u8 = 13;
    const BLUE_CUBES: u8 = 14;

    /* part 1 */
    let possible_games_id_sum: u32 = input
        .lines()
        .filter_map(|line| {
            let (_, (id, rounds)) = parse_game(line).unwrap();

            if rounds
                .iter()
                .flatten()
                .all(|&(count, cube_color)| match cube_color {
                    "red" => count <= RED_CUBES,
                    "green" => count <= GREEN_CUBES,
                    "blue" => count <= BLUE_CUBES,
                    _ => unreachable!(),
                })
            {
                Some(id)
            } else {
                None
            }
        })
        .sum();

    /* part 2 */
    let part_two_sum: u64 = input
        .lines()
        .map(|line| {
            let (_, (_id, rounds)) = parse_game(line).unwrap();

            let (max_red, max_green, max_blue) = rounds.iter().flatten().fold(
                (0, 0, 0),
                |(max_red, max_green, max_blue), &(count, cube_color)| match cube_color {
                    "red" => (max_red.max(count), max_green, max_blue),
                    "green" => (max_red, max_green.max(count), max_blue),
                    "blue" => (max_red, max_green, max_blue.max(count)),
                    _ => unreachable!(),
                },
            );

            max_red as u64 * max_green as u64 * max_blue as u64
        })
        .sum();

    println!(
        "part 1: {} \n part 2: {}",
        possible_games_id_sum, part_two_sum
    );

    (possible_games_id_sum as usize, part_two_sum as usize)
}
