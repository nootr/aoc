use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum MachinePartType {
    X,
    M,
    A,
    S,
}

impl MachinePartType {
    fn parse(c: char) -> Self {
        match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            x => panic!("Unknown machine part type: {}", x),
        }
    }
}

#[derive(Debug)]
enum Rule {
    GreaterThan(MachinePartType, u64, String),
    LessThan(MachinePartType, u64, String),
    Equals(MachinePartType, u64, String),
    Workflow(String),
}

impl Rule {
    fn parse(s: &str) -> Self {
        if "<>=".contains(s.chars().nth(1).unwrap_or('_')) {
            let mut chars = s.chars();
            let machine_part_type = MachinePartType::parse(chars.next().unwrap());
            let comparison = chars.next().unwrap();

            let parts: Vec<&str> = s[2..].split(':').collect();
            let value: u64 = parts[0].parse().unwrap();
            let workflow = parts[1].to_string();

            match comparison {
                '>' => Rule::GreaterThan(machine_part_type, value, workflow),
                '<' => Rule::LessThan(machine_part_type, value, workflow),
                '=' => Rule::Equals(machine_part_type, value, workflow),
                _ => panic!("Unknown comparison: {}", comparison),
            }
        } else {
            Rule::Workflow(s.to_string())
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(line: &str) -> Self {
        let parts: Vec<&str> = line.split('{').collect();
        let name = parts[0].to_string();
        let rules: Vec<Rule> = parts[1][..parts[1].len() - 1]
            .split(',')
            .map(Rule::parse)
            .collect();

        Self { name, rules }
    }
}

#[derive(Debug)]
struct MachinePartSpec {
    machine_part_type: MachinePartType,
    value: u64,
}

#[derive(Debug, Clone)]
struct MachinePartRange {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl MachinePartRange {
    fn full() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    /// Parses a string representing a machine part range of size 1; a single machine part
    fn parse_single(s: &str) -> Self {
        let s = &s[1..s.len() - 1];
        let specs: Vec<MachinePartSpec> = s
            .split(',')
            .map(|s| {
                let parts: Vec<&str> = s.split('=').collect();
                MachinePartSpec {
                    machine_part_type: MachinePartType::parse(parts[0].chars().next().unwrap()),
                    value: parts[1].parse().unwrap(),
                }
            })
            .collect();

        let (x, m, a, s) = specs
            .iter()
            .fold((None, None, None, None), |(x, m, a, s), spec| {
                match spec.machine_part_type {
                    MachinePartType::X => (Some(spec.value), m, a, s),
                    MachinePartType::M => (x, Some(spec.value), a, s),
                    MachinePartType::A => (x, m, Some(spec.value), s),
                    MachinePartType::S => (x, m, a, Some(spec.value)),
                }
            });
        let x = x.unwrap();
        let m = m.unwrap();
        let a = a.unwrap();
        let s = s.unwrap();
        Self {
            x: (x, x),
            m: (m, m),
            a: (a, a),
            s: (s, s),
        }
    }

    fn is_empty(&self) -> bool {
        self.x.0 > self.x.1 || self.m.0 > self.m.1 || self.a.0 > self.a.1 || self.s.0 > self.s.1
    }

    fn split_up_to(&self, machine_part_type: &MachinePartType, split: u64) -> (Self, Self) {
        match machine_part_type {
            MachinePartType::X => (
                Self {
                    x: (self.x.0, min(split - 1, self.x.1)),
                    ..*self
                },
                Self {
                    x: (max(split, self.x.0), self.x.1),
                    ..*self
                },
            ),
            MachinePartType::M => (
                Self {
                    m: (self.m.0, min(split - 1, self.m.1)),
                    ..*self
                },
                Self {
                    m: (max(split, self.m.0), self.m.1),
                    ..*self
                },
            ),
            MachinePartType::A => (
                Self {
                    a: (self.a.0, min(split - 1, self.a.1)),
                    ..*self
                },
                Self {
                    a: (max(split, self.a.0), self.a.1),
                    ..*self
                },
            ),
            MachinePartType::S => (
                Self {
                    s: (self.s.0, min(split - 1, self.s.1)),
                    ..*self
                },
                Self {
                    s: (max(split, self.s.0), self.s.1),
                    ..*self
                },
            ),
        }
    }

    fn execute_workflow(&self, workflow: &Workflow) -> Vec<(String, Self)> {
        let mut split_ranges = vec![];
        let mut remaining_ranges = vec![self.clone()];
        for rule in &workflow.rules {
            let mut new_remaining_ranges = vec![];
            match rule {
                Rule::GreaterThan(machine_part_type, value, next_workflow_name) => {
                    for remaining_range in remaining_ranges {
                        let (low, high) = remaining_range.split_up_to(machine_part_type, value + 1);
                        if !low.is_empty() {
                            new_remaining_ranges.push(low);
                        }
                        if !high.is_empty() {
                            split_ranges.push((next_workflow_name.clone(), high));
                        }
                    }
                }
                Rule::LessThan(machine_part_type, value, next_workflow_name) => {
                    for remaining_range in remaining_ranges {
                        let (low, high) = remaining_range.split_up_to(machine_part_type, *value);
                        if !low.is_empty() {
                            split_ranges.push((next_workflow_name.clone(), low));
                        }
                        if !high.is_empty() {
                            new_remaining_ranges.push(high);
                        }
                    }
                }
                Rule::Equals(machine_part_type, value, next_workflow_name) => {
                    for remaining_range in remaining_ranges {
                        let (low, mid) = remaining_range.split_up_to(machine_part_type, *value);
                        let (mid, high) = mid.split_up_to(machine_part_type, value + 1);
                        if !low.is_empty() {
                            new_remaining_ranges.push(low);
                        }
                        if !mid.is_empty() {
                            split_ranges.push((next_workflow_name.clone(), mid));
                        }
                        if !high.is_empty() {
                            new_remaining_ranges.push(high);
                        }
                    }
                }
                Rule::Workflow(next_workflow_name) => {
                    for remaining_range in remaining_ranges {
                        split_ranges.push((next_workflow_name.clone(), remaining_range));
                    }
                }
            }
            remaining_ranges = new_remaining_ranges;
            if remaining_ranges.is_empty() {
                break;
            }
        }
        split_ranges
    }

    fn total_parts(&self) -> u64 {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }

    fn total_accepted_parts(&self, workflows: &HashMap<String, Workflow>) -> u64 {
        let mut ranges: Vec<(String, MachinePartRange)> = vec![("in".to_string(), self.clone())];
        let mut accepted_count = 0;

        while !ranges.is_empty() {
            let (workflow_name, range) = ranges.remove(0);
            let workflow = workflows.get(&workflow_name).unwrap();
            let new_ranges: Vec<(String, MachinePartRange)> = range
                .execute_workflow(workflow)
                .iter()
                .filter(|(workflow_name, range)| {
                    if *workflow_name == "A" {
                        accepted_count += range.total_parts();
                        false
                    } else {
                        *workflow_name != "R"
                    }
                })
                .cloned()
                .collect();
            ranges.extend(new_ranges);
        }

        accepted_count
    }
}

fn solve_problem_1(input: String) -> u64 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let workflows: HashMap<String, Workflow> = HashMap::from_iter(
        parts[0]
            .lines()
            .map(Workflow::parse)
            .map(|workflow| (workflow.name.clone(), workflow)),
    );
    let machine_parts: Vec<MachinePartRange> = parts[1]
        .lines()
        .map(MachinePartRange::parse_single)
        .collect();

    machine_parts
        .iter()
        .filter(|part| {
            let value = part.total_accepted_parts(&workflows);
            assert!(value <= 1);
            value == 1
        })
        .map(|part| part.x.0 + part.m.0 + part.a.0 + part.s.0)
        .sum()
}

fn solve_problem_2(input: String) -> u64 {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let workflows: HashMap<String, Workflow> = HashMap::from_iter(
        parts[0]
            .lines()
            .map(Workflow::parse)
            .map(|workflow| (workflow.name.clone(), workflow)),
    );
    MachinePartRange::full().total_accepted_parts(&workflows)
}

fn main() {
    let content = fs::read_to_string("../input").expect("Should have been able to read the file");

    let solution = solve_problem_1(content.clone());
    println!("Solution part 1: {}", solution);

    let solution = solve_problem_2(content);
    println!("Solution part 2: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_1_solved() {
        let content = fs::read_to_string("../input-example-1")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_1(content), 19114);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 167409079868000);
    }
}
