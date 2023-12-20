use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, PartialEq, Eq)]
enum ModuleType {
    FlipFlop,
    Conjuction,
    Broadcaster,
}

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    name: String,
    destination_modules: Vec<String>,
    on: bool,
    inputs: HashMap<String, Pulse>,
}

impl Module {
    fn from_line(line: &str) -> Self {
        let parts: Vec<&str> = line.split(" -> ").collect();
        match parts[0].chars().next().unwrap() {
            '%' => Self {
                module_type: ModuleType::FlipFlop,
                name: parts[0][1..].to_string(),
                destination_modules: parts[1].split(", ").map(|s| s.to_string()).collect(),
                on: false,
                inputs: HashMap::new(),
            },
            '&' => Self {
                module_type: ModuleType::Conjuction,
                name: parts[0][1..].to_string(),
                destination_modules: parts[1].split(", ").map(|s| s.to_string()).collect(),
                on: false,
                inputs: HashMap::new(),
            },
            'b' if parts[0] == "broadcaster" => Self {
                module_type: ModuleType::Broadcaster,
                name: parts[0].to_string(),
                destination_modules: parts[1].split(", ").map(|s| s.to_string()).collect(),
                on: false,
                inputs: HashMap::new(),
            },
            _ => panic!("Unknown module: {}", line),
        }
    }
}

fn send_pulse(modules: &mut HashMap<String, Module>) -> (u64, u64) {
    let mut low_count = 1;
    let mut high_count = 0;
    let mut pulse_queue = vec![("button".to_string(), "broadcaster".to_string(), Pulse::Low)];
    while !pulse_queue.is_empty() {
        let (_source, destination, pulse) = pulse_queue.remove(0);
        // NOTE: _source is for debugging purposes, remove when done.
        if !modules.contains_key(&destination) {
            continue;
        }
        match modules.get(&destination).unwrap().module_type {
            ModuleType::FlipFlop => {
                if pulse == Pulse::Low {
                    let (pulse_out, destinations) = {
                        let module = modules.get_mut(&destination).unwrap();
                        module.on = !module.on;
                        let pulse_out = if module.on {
                            high_count += module.destination_modules.len() as u64;
                            Pulse::High
                        } else {
                            low_count += module.destination_modules.len() as u64;
                            Pulse::Low
                        };
                        (pulse_out, module.destination_modules.clone())
                    };
                    for new_destination in destinations {
                        if let Some(destination_module) = modules.get_mut(&new_destination) {
                            destination_module
                                .inputs
                                .insert(destination.clone(), pulse_out.clone());
                        }
                        pulse_queue.push((
                            destination.clone(),
                            new_destination.clone(),
                            pulse_out.clone(),
                        ));
                    }
                }
            }
            ModuleType::Conjuction => {
                let (pulse_out, destinations) = {
                    let module = modules.get_mut(&destination).unwrap();
                    module.on = pulse == Pulse::High;
                    let all_inputs_high = module.inputs.values().all(|p| *p == Pulse::High);
                    let pulse_out = if all_inputs_high {
                        low_count += module.destination_modules.len() as u64;
                        Pulse::Low
                    } else {
                        high_count += module.destination_modules.len() as u64;
                        Pulse::High
                    };
                    (pulse_out, module.destination_modules.clone())
                };
                for new_destination in destinations {
                    if let Some(destination_module) = modules.get_mut(&new_destination) {
                        destination_module
                            .inputs
                            .insert(destination.clone(), pulse_out.clone());
                    }
                    pulse_queue.push((
                        destination.clone(),
                        new_destination.clone(),
                        pulse_out.clone(),
                    ))
                }
            }
            ModuleType::Broadcaster => {
                let destinations = {
                    let module = modules.get_mut(&destination).unwrap();
                    if pulse == Pulse::Low {
                        low_count += module.destination_modules.len() as u64;
                    } else {
                        high_count += module.destination_modules.len() as u64;
                    }
                    module.destination_modules.clone()
                };
                for new_destination in destinations {
                    if let Some(destination_module) = modules.get_mut(&new_destination) {
                        destination_module
                            .inputs
                            .insert(destination.clone(), pulse.clone());
                    }
                    pulse_queue.push((destination.clone(), new_destination.clone(), pulse.clone()))
                }
            }
        }
    }

    (low_count, high_count)
}

fn solve_problem_1(input: String) -> u64 {
    let mut modules: HashMap<String, Module> = HashMap::from_iter(
        input
            .lines()
            .map(Module::from_line)
            .map(|module| (module.name.clone(), module)),
    );
    // Set the initial inputs for each conjuction
    let destinations_map: Vec<(String, Vec<String>)> = {
        modules
            .values()
            .map(|m| (m.name.clone(), m.destination_modules.clone()))
            .collect()
    };
    for (module, destinations) in destinations_map {
        for destination in destinations {
            if let Some(destination_module) = modules.get_mut(&destination) {
                destination_module.inputs.insert(module.clone(), Pulse::Low);
            }
        }
    }

    let mut cumulative_low_count = 0;
    let mut cumulative_high_count = 0;
    for _ in 0..1000 {
        let (low_count, high_count) = send_pulse(&mut modules);
        cumulative_low_count += low_count;
        cumulative_high_count += high_count;
    }
    cumulative_low_count * cumulative_high_count
}

fn solve_problem_2(_input: String) -> u64 {
    0
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
        assert_eq!(solve_problem_1(content), 32000000);
    }

    #[test]
    fn problem_1_solved_b() {
        let content = fs::read_to_string("../input-example-1b")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_1(content), 11687500);
    }

    //#[test]
    //fn problem_2_solved() {
    //    return todo!();
    //    let content = fs::read_to_string("../input-example-2")
    //        .expect("Should have been able to read the file");
    //    assert_eq!(solve_problem_2(content), 5678);
    //}
}
