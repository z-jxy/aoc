use nom::{
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{alpha1, alphanumeric1, newline, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use num::integer::lcm;
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq)]
struct Node {
    ident: String,
    left: String,
    right: String,
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    tuple((alphanumeric1, space1, tag("= ("), take_until(")"), tag(")")))(input).map(
        |(input, (ident, _, _, children, _))| {
            let children = children.split(", ").collect::<Vec<_>>();

            let node = Node {
                ident: ident.to_string(),
                left: children[0].to_string(),
                right: children[1].to_string(),
            };

            (input, node)
        },
    )
}

fn parse_node_map(input: &str) -> IResult<&str, (&str, Vec<Node>)> {
    tuple((
        take_while1(|c: char| c.is_alphabetic()),
        newline,
        newline,
        separated_list1(tag("\n"), parse_node),
    ))(input)
    .map(|(input, res)| {
        let nodes = res.3;

        (input, (res.0, nodes))
    })
}

fn build_tree(nodes: &[Node]) -> HashMap<String, Node> {
    let mut tree = HashMap::new();
    for node in nodes {
        tree.insert(node.ident.clone(), node.clone());
    }
    tree
}

fn build_tree2<'a>(nodes: &'a [Node]) -> HashMap<&'a str, Node> {
    let mut tree = HashMap::new();
    for node in nodes {
        tree.insert(node.ident.as_str(), node.clone());
    }

    tree
}

fn p1(input: &str) {
    let (_, (directions, nodes)) = parse_node_map(&input).unwrap();

    let directions = directions.chars().collect::<Vec<_>>();

    let tree = build_tree(&nodes);

    let mut steps = 0;
    let mut current_ident = "AAA".to_string(); // Assuming "AAA" is the starting node

    while current_ident != "ZZZ" {
        for d in &directions {
            steps += 1;
            if let Some(current) = tree.get(&current_ident) {
                current_ident = match *d {
                    'L' => current.left.clone(),
                    'R' => current.right.clone(),
                    _ => current_ident.clone(), // Handle unexpected direction
                };
            }

            if current_ident == "ZZZ" {
                break;
            }
        }
    }

    println!("Reached 'ZZZ' after {} steps", steps);
}

fn find_cycle_length(
    tree: &HashMap<&str, Node>,
    start_node: &str,
    directions: &[char],
) -> Option<usize> {
    let mut visited = HashMap::new();
    let mut current_ident = start_node;
    let mut step = 0;

    while !current_ident.ends_with("Z") {
        if let Some(&first_seen_step) = visited.get(current_ident) {
            return Some(step - first_seen_step); // Cycle length
        }

        visited.insert(current_ident, step);

        if let Some(current) = tree.get(current_ident) {
            let direction = directions[step % directions.len()];
            current_ident = match direction {
                'L' => &current.left,
                'R' => &current.right,
                _ => current_ident, // Handle unexpected direction
            };
        }

        step += 1;
    }

    None // If path ends in 'Z' without forming a cycle
}

fn p2(input: &[u8]) {
    let split = input.iter().position(|&c| c == b'\n').unwrap();

    let (mut map, mut starts) = ([0u32; 0b11001_11001_11001 + 1], Vec::with_capacity(6));
    let enc = |n: &[u8]| {
        ((n[0] - b'A') as u32) << 10 | ((n[1] - b'A') as u32) << 5 | (n[2] - b'A') as u32
    };
    input[split + 2..].split(|&c| c == b'\n').for_each(|node| {
        map[enc(&node[0..3]) as usize] = enc(&node[7..10]) | enc(&node[12..15]) << 16;
        if node[2] == b'A' {
            starts.push(enc(&node[0..3]));
        }
    });

    println!(
        "{}",
        starts
            .into_iter()
            .map(|node| {
                input[0..split]
                    .iter()
                    .cycle()
                    .scan(node, |node, step| {
                        *node = if step == &b'L' {
                            map[*node as usize] & u16::MAX as u32
                        } else {
                            map[*node as usize] >> 16
                        };
                        Some(*node & 0b11111 == (b'Z' - b'A') as u32)
                    })
                    .position(|node| node)
                    .unwrap()
                    + 1
            })
            .fold(1, lcm)
    );
}

#[aoc::main(08)]
fn main(input: &str) -> (usize, usize) {
    //let input = std::fs::read_to_string("../inputs/08.test").unwrap();

    let (_, (directions, nodes)) = parse_node_map(&input).unwrap();

    let directions = directions.chars().collect::<Vec<_>>();

    let tree = build_tree2(&nodes);

    let mut steps = 0;

    // Find all starting nodes (ending with 'A')
    let mut current_nodes: Vec<&str> = tree
        .keys()
        .filter(|ident| ident.ends_with("A"))
        .cloned()
        .collect();

    println!("starting nodes: {:?}", current_nodes);

    let mut lcm_steps = 1;

    let p2_solution = p2(input.as_bytes());
    //println!("nodes: {:?}", current_nodes);

    (0, 0)
}
