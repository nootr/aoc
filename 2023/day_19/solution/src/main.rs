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
    fn new() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn is_empty(&self) -> bool {
        self.x.0 > self.x.1 || self.m.0 > self.m.1 || self.a.0 > self.a.1 || self.s.0 > self.s.1
    }

    fn split_up_to(&self, machine_part_type: &MachinePartType, split: u64) -> (Self, Self) {
        match machine_part_type {
            MachinePartType::X => (
                Self {
                    x: (self.x.0, split - 1),
                    ..*self
                },
                Self {
                    x: (split, self.x.1),
                    ..*self
                },
            ),
            MachinePartType::M => (
                Self {
                    m: (self.m.0, split - 1),
                    ..*self
                },
                Self {
                    m: (split, self.m.1),
                    ..*self
                },
            ),
            MachinePartType::A => (
                Self {
                    a: (self.a.0, split - 1),
                    ..*self
                },
                Self {
                    a: (split, self.a.1),
                    ..*self
                },
            ),
            MachinePartType::S => (
                Self {
                    s: (self.s.0, split - 1),
                    ..*self
                },
                Self {
                    s: (split, self.s.1),
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
}

#[derive(Debug)]
struct MachinePart {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl MachinePart {
    fn parse(s: &str) -> Self {
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
        Self {
            x: x.unwrap(),
            m: m.unwrap(),
            a: a.unwrap(),
            s: s.unwrap(),
        }
    }

    fn test(&self, workflows: &HashMap<String, Workflow>) -> bool {
        let mut workflow_name = "in".to_string();
        loop {
            if workflow_name == "A" {
                return true;
            }
            if workflow_name == "R" {
                return false;
            }
            let workflow = workflows.get(&workflow_name).unwrap();
            for rule in &workflow.rules {
                match rule {
                    Rule::GreaterThan(machine_part_type, value, next_workflow_name) => {
                        if self.value(machine_part_type) > *value {
                            workflow_name = next_workflow_name.clone();
                            break;
                        }
                    }
                    Rule::LessThan(machine_part_type, value, next_workflow_name) => {
                        if self.value(machine_part_type) < *value {
                            workflow_name = next_workflow_name.clone();
                            break;
                        }
                    }
                    Rule::Equals(machine_part_type, value, next_workflow_name) => {
                        if self.value(machine_part_type) == *value {
                            workflow_name = next_workflow_name.clone();
                            break;
                        }
                    }
                    Rule::Workflow(next_workflow_name) => {
                        workflow_name = next_workflow_name.clone();
                    }
                }
            }
        }
    }

    fn value(&self, machine_part_type: &MachinePartType) -> u64 {
        match machine_part_type {
            MachinePartType::X => self.x,
            MachinePartType::M => self.m,
            MachinePartType::A => self.a,
            MachinePartType::S => self.s,
        }
    }

    fn total_value(&self) -> u64 {
        self.x + self.m + self.a + self.s
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
    let machine_parts: Vec<MachinePart> = parts[1].lines().map(MachinePart::parse).collect();

    machine_parts
        .iter()
        .filter(|part| part.test(&workflows))
        .map(|part| part.total_value())
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

    let mut ranges: Vec<(String, MachinePartRange)> =
        vec![("in".to_string(), MachinePartRange::new())];
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
