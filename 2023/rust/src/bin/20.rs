use std::collections::{HashMap, VecDeque};

use hashbrown::HashSet;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::opt,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use num::Integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop(u8),
    Conjunction(HashMap<String, Pulse>),
    Broadcast,
}

#[derive(Debug, Clone)]
struct Destinations(Vec<String>);

#[derive(Debug, Clone)]
struct Module(ModuleType, Destinations);

impl Module {
    fn process_pulse(&mut self, sender: &str, pulse: Pulse) -> Option<(Destinations, Pulse)> {
        match self.0 {
            ModuleType::FlipFlop(state) => {
                if pulse == Pulse::High {
                    return None;
                }

                match state {
                    0 => {
                        self.0 = ModuleType::FlipFlop(1);
                        return Some((self.1.clone(), Pulse::High));
                    }
                    1 => {
                        self.0 = ModuleType::FlipFlop(0);
                        return Some((self.1.clone(), Pulse::Low));
                    }
                    _ => unreachable!(),
                }
            }
            ModuleType::Conjunction(ref mut state) => {
                state.insert(sender.to_string(), pulse);
                let pulse = match state.iter().all(|(_, p)| p == &Pulse::High) {
                    true => Pulse::Low,
                    false => Pulse::High,
                };
                return Some((self.1.clone(), pulse));
            }
            ModuleType::Broadcast => {
                return Some((self.1.clone(), pulse));
            }
        }
    }
}

fn parse_config(input: &str) -> IResult<&str, HashMap<String, Module>> {
    separated_list1(
        newline,
        tuple((
            opt(alt((tag("%"), tag("&")))),
            alpha1,
            tag(" -> "),
            separated_list1(tag(", "), alpha1),
        )),
    )(input)
    .map(|(input, res)| {
        let mut modules = HashMap::new();
        for (t, name, _, dests) in res {
            let (name, destinations) = (
                name.to_string(),
                Destinations(dests.iter().map(|s| s.to_string()).collect()),
            );
            let module_type = match t {
                Some("%") => ModuleType::FlipFlop(0),
                Some("&") => ModuleType::Conjunction(HashMap::new()),
                None => ModuleType::Broadcast,
                _ => panic!("Unknown module type"),
            };

            let module = Module(module_type, destinations);

            modules.insert(name, module);
        }
        (input, modules)
    })
}

fn push_button(modules: &mut HashMap<String, Module>, count: usize) -> usize {
    let mut queue = VecDeque::new();
    let mut pulses = vec![];
    for _ in 0..count {
        queue.push_back((
            "button".to_string(),
            ("broadcaster".to_string(), Pulse::Low),
        ));
        pulses.push(Pulse::Low);

        while let Some((sender, (receiver, recv_pulse))) = queue.pop_front() {
            if let Some(module) = modules.get_mut(&receiver) {
                if let Some((destinations, send_pulse)) = module.process_pulse(&sender, recv_pulse)
                {
                    for dest in destinations.0 {
                        pulses.push(send_pulse.clone());
                        queue.push_back((receiver.clone(), (dest.clone(), send_pulse.clone())));
                    }
                }
            }
        }
    }

    let low = pulses.iter().filter(|p| p == &&Pulse::Low).count();
    let high = pulses.len() - low;

    low * high
}

fn p2(modules: &mut HashMap<String, Module>) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashMap::new();
    let check = modules.clone();
    let feed = check
        .iter()
        .find(|(_, module)| module.1 .0.contains(&"rx".to_string()))
        .unwrap();
    let mut cycle_lengths = HashMap::new();
    for (name, module) in modules.iter() {
        if module.1 .0.contains(feed.0) {
            seen.insert(name.clone(), 0);
        }
    }

    let mut pressed = 0;
    loop {
        queue.push_back((
            "button".to_string(),
            ("broadcaster".to_string(), Pulse::Low),
        ));
        pressed += 1;

        while let Some((sender, (receiver, recv_pulse))) = queue.pop_front() {
            if receiver == *feed.0 && recv_pulse == Pulse::High {
                seen.entry(sender.clone())
                    .and_modify(|c| *c += 1)
                    .or_insert(1);

                if !cycle_lengths.contains_key(&sender) {
                    cycle_lengths.insert(sender.clone(), pressed);
                }
            }

            if seen.values().all(|c| *c != 0) {
                // take the lcm of all cycle lengths (+ 1 for the initial button press)
                let p2 = cycle_lengths.values().fold(1, |acc, c| acc.lcm(c));
                return p2;
            }

            if let Some(module) = modules.get_mut(&receiver) {
                if let Some((destinations, send_pulse)) = module.process_pulse(&sender, recv_pulse)
                {
                    for dest in destinations.0 {
                        queue.push_back((receiver.clone(), (dest.clone(), send_pulse.clone())));
                    }
                }
            }
        }
    }
}

fn p1(modules: &mut HashMap<String, Module>) -> usize {
    push_button(modules, 1000)
}

#[aoc::main(20)]
fn main(input: &str) -> (usize, usize) {
    let (_, mut modules) = parse_config(&input).unwrap();
    // get a list of all conjunction modules
    let conjunction_modules = modules
        .iter()
        .filter(|(_, m)| match m.0 {
            ModuleType::Conjunction(_) => true,
            _ => false,
        })
        .map(|(k, _)| k.clone())
        .collect::<Vec<_>>();

    let mut dependencies: HashMap<String, HashSet<String>> = HashMap::new();
    // if any module has a destination that is a conjunction module, add it to the dependencies
    for (name, module) in &modules {
        let destinations = &module.1;
        for dest in &destinations.0 {
            if conjunction_modules.contains(&dest) {
                let (cm, _) = modules.get_key_value(&dest.clone()).unwrap();
                // rust borrowing rules mutable references rule ):
                dependencies
                    .entry(cm.clone())
                    .and_modify(|d| {
                        d.insert(name.clone());
                    })
                    .or_insert(HashSet::from([name.clone()]));
            }
        }
    }
    // set the initial state of all modules to watch for the conjunction modules to low
    for (name, d) in dependencies.iter() {
        let cm = modules.get_mut(name).unwrap();
        if let ModuleType::Conjunction(ref mut state) = cm.0 {
            for dep in d {
                state.insert(dep.clone(), Pulse::Low);
            }
        }
    }

    (p1(&mut modules), p2(&mut modules))
}
