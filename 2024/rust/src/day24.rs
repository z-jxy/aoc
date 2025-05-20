use std::{collections::HashMap, mem, rc::Rc};

use aoc_runner_derive::{aoc, aoc_generator};

type Input = (Wires, Vec<Instance>);
type Wires = HashMap<Rc<str>, u8>;

#[derive(Debug, Clone)]
struct Instance {
    lhs: Rc<str>,
    rhs: Rc<str>,
    op: String,
    out: Rc<str>,
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Input {
    let (a, b) = input.split_once("\n\n").unwrap();

    let mut wires = HashMap::new();
    let mut instances = vec![];

    for line in a.lines() {
        let (l, r) = line.split_once(": ").unwrap();

        wires.insert(l.into(), r.parse::<u8>().unwrap());
    }

    for line in b.lines() {
        let (l, out) = line.split_once(" -> ").unwrap();
        let mut w_split = l.split_whitespace();
        let lhs = w_split.next().unwrap().into();
        let op = w_split.next().unwrap().to_string();
        let rhs = w_split.next().unwrap().into();

        instances.push(Instance {
            lhs,
            rhs,
            op,
            out: out.into(),
        });
    }

    (wires, instances)
}

#[inline(always)]
fn and(a: u8, b: u8) -> bool {
    a & b == 1
}

#[inline(always)]
fn or(a: u8, b: u8) -> bool {
    a | b == 1
}

#[inline(always)]
fn xor(a: u8, b: u8) -> bool {
    a ^ b == 1
}

fn extract_binary_number(wires: &Wires, prefix: &str) -> usize {
    let mut bits: Vec<_> = wires
        .iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .collect();

    bits.sort_by(|(a, _), (b, _)| {
        let a_num: usize = a[prefix.len()..].parse().unwrap();
        let b_num: usize = b[prefix.len()..].parse().unwrap();
        b_num.cmp(&a_num)
    });

    let mut result = 0;
    for (_, &bit) in bits.iter() {
        result = (result << 1) | (bit as usize);
    }
    result
}

fn simulate_circuit(wires: &HashMap<Rc<str>, u8>, input: &Vec<Instance>) -> HashMap<Rc<str>, u8> {
    let mut results = HashMap::new();

    for (k, v) in wires.iter() {
        results.insert(k.clone(), *v);
    }

    // Keep track of which instances still need to be processed
    let mut pending_instances: Vec<_> = input.iter().collect();

    // Continue until we've processed all instances or can't make progress
    while !pending_instances.is_empty() {
        // Try to process each remaining instance
        pending_instances.retain(|instance| {
            // Check if we have both inputs available
            if let (Some(&lhs), Some(&rhs)) =
                (results.get(&instance.lhs), results.get(&instance.rhs))
            {
                // We can process this instance
                let out = match instance.op.as_str() {
                    "AND" => and(lhs, rhs),
                    "OR" => or(lhs, rhs),
                    "XOR" => xor(lhs, rhs),
                    _ => unreachable!(),
                };

                results.insert(instance.out.clone(), out as u8);
                false // Remove this instance from pending
            } else {
                true // Keep this instance in pending
            }
        });
    }

    results
}

#[aoc(day24, part1)]
fn part1((wires, input): &Input) -> usize {
    let results = simulate_circuit(wires, input);
    // println!("{:?}", input);

    // combine the bits of all results that start with 'z'
    extract_binary_number(&results, "z")
}

fn gate_search(input: &[Instance], lhs: &str, rhs: &str, op: &str) -> Option<Rc<str>> {
    input
        .iter()
        .find(|gate| {
            gate.op == op
                && ((*gate.lhs == *lhs && *gate.rhs == *rhs)
                    || (*gate.lhs == *rhs && *gate.rhs == *lhs))
        })
        .map(|gate| gate.out.clone())
}

#[aoc(day24, part2)]
fn part2((_wires, input): &Input) -> String {
    let mut swapped = Vec::new();
    let mut last_carry: Option<Rc<str>> = None;

    for i in 0..45 {
        let pos = format!("{:02}", i);
        let x = format!("x{}", pos);
        let y = format!("y{}", pos);

        // First half-adder stage
        let mut half_adder_1_sum = gate_search(input, &x, &y, "XOR");
        let mut half_adder_1_carry = gate_search(input, &x, &y, "AND");

        let mut full_adder_carry = None;
        let mut half_adder_2_sum = None;

        if let Some(prev_carry) = &last_carry {
            // Try to find second half-adder components
            let mut half_adder_2_carry = if let Some(ha1_sum) = &half_adder_1_sum {
                let result = gate_search(input, prev_carry, ha1_sum, "AND");
                if result.is_none() {
                    // If we can't find it, try swapping the half adder outputs
                    mem::swap(&mut half_adder_1_sum, &mut half_adder_1_carry);
                    if let (Some(s), Some(c)) = (&half_adder_1_sum, &half_adder_1_carry) {
                        swapped.extend([s.clone(), c.clone()]);
                    }
                    gate_search(
                        input,
                        prev_carry,
                        half_adder_1_sum.as_deref().unwrap(),
                        "AND",
                    )
                } else {
                    result
                }
            } else {
                None
            };

            // Find XOR of previous carry and first half-adder sum
            if let Some(ha1_sum) = &half_adder_1_sum {
                half_adder_2_sum = gate_search(input, prev_carry, ha1_sum, "XOR");
            }

            // Check for z-register misplacements
            if let Some(ha1_sum) = &half_adder_1_sum {
                if ha1_sum.starts_with('z') {
                    if let Some(ha2_sum) = &half_adder_2_sum {
                        swapped.extend([ha1_sum.clone(), ha2_sum.clone()]);
                        mem::swap(&mut half_adder_1_sum, &mut half_adder_2_sum);
                    }
                }
            }

            if let Some(ha1_carry) = &half_adder_1_carry {
                if ha1_carry.starts_with('z') {
                    if let Some(ha2_sum) = &half_adder_2_sum {
                        swapped.extend([ha1_carry.clone(), ha2_sum.clone()]);
                        mem::swap(&mut half_adder_1_carry, &mut half_adder_2_sum);
                    }
                }
            }

            if let Some(ha2_carry) = &half_adder_2_carry {
                if ha2_carry.starts_with('z') {
                    if let Some(ha2_sum) = &half_adder_2_sum {
                        swapped.extend([ha2_carry.clone(), ha2_sum.clone()]);
                        mem::swap(&mut half_adder_2_carry, &mut half_adder_2_sum);
                    }
                }
            }

            // Find OR of the carries
            if let (Some(ha1_carry), Some(ha2_carry)) = (&half_adder_1_carry, &half_adder_2_carry) {
                full_adder_carry = gate_search(input, ha1_carry, ha2_carry, "OR");
            }
        }

        // Check if full adder carry is going to wrong z-register
        if let Some(carry) = &full_adder_carry {
            if carry.starts_with('z') && **carry != *"z45" {
                if let Some(ha2_sum) = &half_adder_2_sum {
                    swapped.extend([carry.clone(), ha2_sum.clone()]);
                    mem::swap(&mut full_adder_carry, &mut half_adder_2_sum);
                }
            }
        }

        // Update carry for next iteration
        match last_carry {
            Some(_) => last_carry = full_adder_carry,
            None => last_carry = full_adder_carry,
        }
    }

    // Sort and join the swapped wires
    let mut result: Vec<_> = swapped.iter().cloned().collect();
    result.sort();
    result.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 2024);
    }

    #[test]
    fn part2_example() {
        // example for p2 wasn't that good
        assert_eq!(
            part2(&parse(include_str!("../input/2024/day24.txt"))),
            "cqm,mps,vcv,vjv,vwp,z13,z19,z25"
        );
    }
}
