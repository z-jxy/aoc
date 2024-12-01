use std::{cmp::Ordering, collections::HashMap};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline},
    combinator::opt,
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct WorkflowId(String);

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
    result: WorkflowId,
}

#[derive(Debug, Clone)]
struct Condition(Ordering, usize);

#[derive(Debug, Clone)]
struct Rule {
    category: String,
    condition: Condition,
    next: WorkflowId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
#[derive(Debug, Clone, PartialEq)]
enum Status {
    Accepted,
    Rejected,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct MinMax {
    min: usize,
    max: usize,
}

fn parse_rule(input: &str) -> IResult<&str, (&str, Option<(Ordering, &str, &str)>)> {
    tuple((
        alpha1,
        opt(tuple((
            alt((
                tag("<").map(|_| Ordering::Less),
                tag(">").map(|_| Ordering::Greater),
            )),
            terminated(digit1, tag(":")),
            alpha1,
        ))),
    ))(input)
    .map(|(input, res)| (input, res))
}

fn parse_workflows(input: &str) -> IResult<&str, HashMap<WorkflowId, Workflow>> {
    separated_list1(
        tag("\n"),
        tuple((
            alpha1,
            delimited(tag("{"), separated_list1(tag(","), parse_rule), tag("}")),
        )),
    )(input)
    .map(|(input, res)| {
        let mut workflows = HashMap::new();

        for (workflow_id, mut rules) in res {
            let last = rules.remove(rules.len() - 1).0;

            let mut workflow = Workflow {
                rules: vec![],
                result: WorkflowId(last.to_string()),
            };

            for rule in rules {
                let ident = rule.0;
                let (condition, value, next_step) = rule.1.unwrap();
                let condition = Condition(condition, value.parse::<usize>().unwrap());
                workflow.rules.push(Rule {
                    category: ident.to_string(),
                    condition,
                    next: WorkflowId(next_step.to_string()),
                });
            }

            workflows.insert(WorkflowId(workflow_id.to_string()), workflow);
        }

        (input, workflows)
    })
}

fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(
        tag("\n"),
        delimited(
            tag("{"),
            separated_list1(tag(","), tuple((alpha1, tag("="), digit1))),
            tag("}"),
        ),
    )(input)
    .map(|(input, res)| {
        let mut parts = vec![];
        for part in res {
            let x = part[0].2.parse::<usize>().unwrap();
            let m = part[1].2.parse::<usize>().unwrap();
            let a = part[2].2.parse::<usize>().unwrap();
            let s = part[3].2.parse::<usize>().unwrap();
            parts.push(Part { x, m, a, s });
        }

        (input, parts)
    })
}

fn parse_workflows_and_parts(
    input: &str,
) -> IResult<&str, (HashMap<WorkflowId, Workflow>, Vec<Part>)> {
    tuple((parse_workflows, newline, newline, parse_parts))(input)
        .map(|(input, (workflows, _, _, parts))| (input, (workflows, parts)))
}

fn process_workflow(
    part: &Part,
    workflow_id: &WorkflowId,
    workflows: &HashMap<WorkflowId, Workflow>,
) -> Status {
    match workflow_id.0.as_str() {
        "A" => return Status::Accepted,
        "R" => return Status::Rejected,
        _ => {}
    }

    let workflow = workflows.get(&workflow_id).unwrap();
    for rule in &workflow.rules {
        let part_value = match rule.category.as_str() {
            "x" => part.x,
            "m" => part.m,
            "a" => part.a,
            "s" => part.s,
            _ => panic!("Unknown category"),
        };
        if rule.condition.0 == part_value.cmp(&rule.condition.1) {
            return process_workflow(part, &rule.next, workflows);
        }
    }

    process_workflow(part, &workflow.result, workflows)
}

fn p1(workflows: &HashMap<WorkflowId, Workflow>, parts: &[Part]) -> usize {
    let total = parts
        .iter()
        .filter(|part| {
            process_workflow(&part, &WorkflowId("in".to_string()), &workflows) == Status::Accepted
        })
        .map(|part| part.x + part.m + part.a + part.s)
        .sum::<usize>();

    total
}

fn count(
    ranges: &HashMap<String, MinMax>,
    workflows: &HashMap<WorkflowId, Workflow>,
    workflow_id: &WorkflowId,
) -> usize {
    match workflow_id.0.as_str() {
        "R" => return 0, // reject everything
        "A" => {
            return ranges
                .values()
                .map(|min_max| min_max.max - min_max.min + 1)
                .product()
        }
        _ => {}
    }
    let mut total = 0;
    let mut new_ranges = ranges.clone();
    let workflow = workflows.get(&workflow_id).unwrap();

    for rule in &workflow.rules {
        let Condition(ord, n) = rule.condition;

        let min_max = new_ranges.get(&rule.category).unwrap();
        let (low, high) = (min_max.min, min_max.max);

        let (t, f) = match ord {
            Ordering::Less => {
                if high < n - 1 {
                    // edge case
                    ((low, high), (n, high))
                } else {
                    ((low, n - 1), (n, high))
                }
            }
            _ => ((n + 1, high), (low, n)),
        };

        if t.0 <= t.1 {
            // clone the current ranges and update range
            let mut new_ranges = new_ranges.clone();
            new_ranges.insert(rule.category.to_owned(), MinMax { min: t.0, max: t.1 });
            total += count(&new_ranges, workflows, &rule.next);
        }

        if f.0 <= f.1 {
            new_ranges.insert(rule.category.to_owned(), MinMax { min: f.0, max: f.1 });
        } else {
            break;
        }
    }

    total += count(&new_ranges, workflows, &workflow.result);

    total
}

fn p2(workflows: &HashMap<WorkflowId, Workflow>) -> usize {
    let initial_range = MinMax { min: 1, max: 4000 };
    let ranges = HashMap::from([
        ("x".to_string(), initial_range.clone()),
        ("m".to_string(), initial_range.clone()),
        ("a".to_string(), initial_range.clone()),
        ("s".to_string(), initial_range.clone()),
    ]);

    count(&ranges, &workflows, &WorkflowId("in".to_string()))
}

#[aoc::main(19)]
fn main(input: &str) -> (usize, usize) {
    let (_, (workflows, parts)) = parse_workflows_and_parts(&input).unwrap();
    let p1 = p1(&workflows, &parts);
    let p2 = p2(&workflows);
    (p1, p2)
}
