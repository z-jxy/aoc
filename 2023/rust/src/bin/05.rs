use std::collections::HashSet;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, newline, space1},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Default, Debug)]
struct Mapping(usize, usize, usize);

struct SeedRange {
    start: usize,
    end: usize,
}

impl SeedRange {
    fn new(start: usize, end: usize) -> Self {
        SeedRange { start, end }
    }

    fn overlaps(&self, other: &SeedRange) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    fn merge(&self, other: &SeedRange) -> SeedRange {
        SeedRange::new(self.start.min(other.start), self.end.max(other.end))
    }
}

fn insert_range(ranges: &mut Vec<SeedRange>, new_range: SeedRange) {
    for range in ranges.iter_mut() {
        if range.overlaps(&new_range) {
            *range = range.merge(&new_range);
            return;
        }
    }
    ranges.push(new_range);
}

fn parse_map_line(input: &str) -> IResult<&str, (usize, usize, usize)> {
    tuple((
        terminated(digit1, space1),
        terminated(digit1, space1),
        digit1,
    ))(input)
    .map(|(next, res)| {
        let res = (
            res.0.parse::<usize>().unwrap(),
            res.1.parse::<usize>().unwrap(),
            res.2.parse::<usize>().unwrap(),
        );

        (next, res)
    })
}
fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    tuple((
        tag("seeds: "),
        separated_list1(space1, digit1),
        terminated(take_while1(|c| c != ':'), tag(":\n")),
        separated_list1(
            newline,
            tuple((
                separated_list1(newline, parse_map_line),
                terminated(take_while1(|c| c != ':'), tag(":")),
            )),
        ),
        newline,
        separated_list1(newline, parse_map_line),
    ))(input)
    .map(|(next, (_, seeds, _, mappings, _, last_mapping))| {
        let mut almanac = Almanac::default();

        let mut ranges = Vec::new();
        for seed_range in seeds.chunks(2) {
            if let (Some(start_str), Some(range_str)) = (seed_range.get(0), seed_range.get(1)) {
                let start = start_str.parse::<usize>().unwrap();
                let range = range_str.parse::<usize>().unwrap();

                // for part 1
                almanac.seeds.insert(start);
                almanac.seeds.insert(range);

                insert_range(&mut ranges, SeedRange::new(start, (start + range) - 1));
            }
        }

        almanac.range_of_seeds = ranges;

        let mut entries = mappings.iter();
        while let Some((mappings, _)) = entries.next() {
            almanac.mappings.push(
                mappings
                    .iter()
                    .map(|(dest, src, range)| Mapping(*dest, *src, *range))
                    .collect(),
            );
        }

        almanac.mappings.push(
            last_mapping
                .iter()
                .map(|(humidity, location, range)| Mapping(*humidity, *location, *range))
                .collect(),
        );

        (next, almanac)
    })
}

#[derive(Default)]
struct Almanac {
    seeds: HashSet<usize>,
    range_of_seeds: Vec<SeedRange>,
    mappings: Vec<Vec<Mapping>>, // [Vec<Mapping>; 7],
}

impl Almanac {
    fn try_convert_src_to_dest_from_mappings(seed: usize, mappings: &[Mapping]) -> Option<usize> {
        for mapping in mappings {
            // Check if the seed is within the source range of the mapping
            if seed >= mapping.1 && seed < mapping.1 + mapping.2 {
                // Calculate the offset of the seed from the start of the source range
                let offset = seed - mapping.1;
                // Apply the same offset to the destination start
                return Some(mapping.0 + offset);
            }
        }

        None
    }
}

#[aoc::main(05)]
fn main(input: &str) -> (usize, usize) {
    let (_, almanac) = parse_almanac(&input).unwrap();

    let mappings = almanac.mappings;

    let p1 = &almanac
        .seeds
        .iter()
        .map(|seed| {
            let location = mappings.iter().fold(*seed, |acc, mappings| {
                if let Some(res) = Almanac::try_convert_src_to_dest_from_mappings(acc, mappings) {
                    return res;
                }
                acc
            });
            (seed, location)
        })
        .min_by_key(|(_, location)| *location)
        .unwrap();

    let mut current_ranges = almanac
        .range_of_seeds
        .iter()
        .map(|x| (x.start, x.end))
        .collect::<HashSet<_>>();

    for mapping in mappings.iter() {
        let mut next_ranges = HashSet::new();
        for range in current_ranges.iter() {
            let mut range_splits = vec![*range];

            for line in mapping.iter() {
                range_splits = range_splits
                    .iter()
                    .flat_map(|range_split| {
                        let (range_start, range_end) = (range_split.0, range_split.1);
                        let (line_start, line_end) = (line.1, line.1 + line.2 - 1);

                        let mut line_splits = Vec::new();
                        if range_end > line_start && line_end >= range_start {
                            if range_start < line_start {
                                line_splits.push((range_start, line_start - 1));
                            }

                            next_ranges.insert((
                                line.0 + (range_start.max(line_start) - line_start), // overlap start
                                line.0 + (range_end.min(line_end) - line_start),     // overlap end
                            ));

                            if range_end > line_end {
                                line_splits.push((line_end + 1, range_end));
                            }
                        } else {
                            line_splits.push((range_start, range_end));
                        }

                        line_splits
                    })
                    .collect::<Vec<_>>();
            }
            next_ranges.extend(range_splits.into_iter());
        }
        current_ranges = next_ranges.into_iter().collect();
    }

    let p2 = current_ranges.iter().map(|x| x.0).min().unwrap();

    (p1.1, p2)
}
