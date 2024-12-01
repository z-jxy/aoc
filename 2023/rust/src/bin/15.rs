use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::digit1,
    combinator::opt,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

fn hash(s: &[u8]) -> u16 {
    s.iter().fold(0, |acc, c| ((acc + *c as u16) * 17) % 256)
}

#[derive(Debug)]
struct Seq {
    word: String,
    box_hash: u16,
    lens: Option<u8>,
}

fn parse_entries(input: &str) -> IResult<&str, Vec<Seq>> {
    separated_list0(
        tag(","),
        tuple((
            take_while(|c: char| c.is_ascii_alphabetic()),
            alt((tag("="), tag("-"))),
            opt(digit1),
        )),
    )(input)
    .map(|(input, res)| {
        let seqs = res
            .iter()
            .map(|(word, _, len)| Seq {
                word: word.to_string(),
                box_hash: hash(word.as_bytes()),
                lens: len.map(|len| len.parse::<u8>().unwrap()),
            })
            .collect::<Vec<_>>();

        (input, seqs)
    })
}

fn p1(input: &str) -> usize {
    let words_to_hash = input.split(',').map(|s| s.as_bytes()).collect::<Vec<_>>();
    words_to_hash
        .iter()
        .map(|w| hash(w) as usize)
        .sum::<usize>()
}

fn p2(input: &str) -> usize {
    let (_, res) = parse_entries(&input).unwrap();
    let mut boxes = vec![Vec::new(); 256];

    for s in res {
        let b = s.box_hash;
        let lens_slots = boxes.get_mut(b as usize).unwrap();

        if let Some(lens) = s.lens {
            if let Some(pos) = lens_slots.iter().position(|(label, _)| label == &s.word) {
                lens_slots[pos].1 = lens;
            } else {
                lens_slots.push((s.word, lens));
            }
        } else {
            if let Some(pos) = lens_slots
                .iter_mut()
                .position(|(label, _)| label == &s.word)
            {
                lens_slots.remove(pos);
            }
        }
    }

    let total_focusing_power = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(lens_slot, (_, focal_length))| {
                    (i + 1) * (lens_slot + 1) * (*focal_length as usize)
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    assert!(total_focusing_power == 212763);

    total_focusing_power
}

#[aoc::main(15)]
fn main(input: &str) -> (usize, usize) {
    (p1(input), p2(input))
}
