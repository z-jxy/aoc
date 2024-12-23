use core::str;
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

type NodeK = Rc<str>;

type AdjList = HashMap<NodeK, HashSet<NodeK>>;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day23)]
fn parse(input: &str) -> AdjList {
    let mut adj_list: AdjList = HashMap::new();

    input.lines().for_each(|l| {
        let (l, r) = l.split_once('-').unwrap();

        let l: NodeK = l.into();
        let r: NodeK = r.into();

        adj_list
            .entry(l.clone())
            .or_insert_with(HashSet::new)
            .insert(r.clone());

        adj_list.entry(r).or_insert_with(HashSet::new).insert(l);
    });

    adj_list
}

fn find_multiplayer_lobby(graph: &AdjList) -> Vec<HashSet<NodeK>> {
    let mut triangles = Vec::new();
    let mut seen = HashSet::new();
    let nodes: Vec<_> = graph.keys().collect();

    for i in 0..nodes.len() {
        let node1 = nodes[i];
        for neighbor1 in graph[node1].iter() {
            if neighbor1 <= node1 {
                continue; // skip if we've already considered this pair
            }
            for neighbor2 in graph[node1].iter() {
                if neighbor2 <= neighbor1 {
                    continue; // skip if we've already considered this triple
                }

                // check if neighbor1 and neighbor2 are connected
                if graph[neighbor1].contains(neighbor2) {
                    let mut triangle_nodes =
                        vec![node1.clone(), neighbor1.clone(), neighbor2.clone()];
                    triangle_nodes.sort();
                    let triangle_key = triangle_nodes.join(",");

                    if !seen.contains(&triangle_key) {
                        seen.insert(triangle_key);
                        let mut triangle = HashSet::new();
                        triangle.insert(node1.clone());
                        triangle.insert(neighbor1.clone());
                        triangle.insert(neighbor2.clone());
                        triangles.push(triangle);
                    }
                }
            }
        }
    }

    triangles
}

fn find_maximum_clique(graph: &AdjList) -> HashSet<NodeK> {
    let mut max_clique = HashSet::new();
    let mut current = HashSet::new();
    let mut candidates: Vec<_> = graph.keys().cloned().collect();
    candidates.sort();

    fn bronk(
        graph: &AdjList,
        current: &mut HashSet<NodeK>,
        candidates: &mut Vec<NodeK>,
        max_clique: &mut HashSet<NodeK>,
        start_idx: usize,
    ) {
        if current.len() > max_clique.len() {
            max_clique.clear();
            max_clique.extend(current.iter().cloned());
        }

        for i in start_idx..candidates.len() {
            // check if candidate connects to all current members
            if current.iter().all(|u| graph[u].contains(&candidates[i])) {
                current.insert(candidates[i].clone());
                if current.len() > max_clique.len() {
                    max_clique.clear();
                    max_clique.extend(current.iter().cloned());
                }
                bronk(graph, current, candidates, max_clique, i + 1);
                current.remove(&candidates[i]);
            }
        }
    }

    bronk(graph, &mut current, &mut candidates, &mut max_clique, 0);
    max_clique
}

#[aoc(day23, part1)]
fn part1(adj_list: &AdjList) -> usize {
    find_multiplayer_lobby(&adj_list).iter().fold(0, |acc, s| {
        if s.iter().any(|node| node.starts_with("t")) {
            acc + 1
        } else {
            acc
        }
    })
}

#[aoc(day23, part2)]
fn part2(adj_list: &AdjList) -> String {
    let max_clique = find_maximum_clique(&adj_list);

    let mut password: Vec<String> = max_clique.iter().map(|s| s.to_string()).collect();
    password.sort();

    password.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), "co,de,ka,ta");
    }
}
