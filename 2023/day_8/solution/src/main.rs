use std::collections::HashMap;
use std::fs;

// Euclid's algorithm
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn lcm(n: u64, m: u64) -> u64 {
    n * m / gcd(n, m)
}

fn solve_problem_1(input: String) -> u64 {
    let mut left_map = HashMap::new();
    let mut right_map = HashMap::new();

    // Parse input
    let lines: Vec<&str> = input.lines().collect();
    let directions = lines[0].chars().collect::<Vec<char>>();
    let lines = &lines[2..];
    for line in lines.iter() {
        let id = line[..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();

        left_map.insert(id.clone(), left);
        right_map.insert(id, right);
    }

    // Traverse the graph
    let mut steps: u64 = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        let index = (steps % directions.len() as u64) as usize;
        steps += 1;
        current = match directions[index] {
            'L' => left_map.get(current).unwrap(),
            'R' => right_map.get(current).unwrap(),
            _ => panic!("Invalid direction"),
        };
    }
    steps
}

fn solve_problem_2(input: String) -> u64 {
    let mut left_map = HashMap::new();
    let mut right_map = HashMap::new();

    // Parse input
    let lines: Vec<&str> = input.lines().collect();
    let directions = lines[0].chars().collect::<Vec<char>>();
    let lines = &lines[2..];
    let mut starting_nodes = vec![];
    for line in lines.iter() {
        let id = line[..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();

        left_map.insert(id.clone(), left);
        right_map.insert(id.clone(), right);

        if id.ends_with('A') {
            starting_nodes.push(id);
        }
    }

    // Traverse the graph
    let mut steps_per_starting_node: Vec<u64> = vec![];

    for starting_node in starting_nodes {
        let mut steps: u64 = 0;
        let mut current = &starting_node;
        while !current.ends_with('Z') {
            let index = (steps % directions.len() as u64) as usize;
            steps += 1;
            current = match directions[index] {
                'L' => left_map.get(current).unwrap(),
                'R' => right_map.get(current).unwrap(),
                _ => panic!("Invalid direction"),
            };
        }

        // Check if the least common multiple is a valid way to get the answer
        let mut verify_steps: u64 = 0;
        while !current.ends_with('Z') || verify_steps == 0 {
            let index = ((steps + verify_steps) % directions.len() as u64) as usize;
            verify_steps += 1;
            current = match directions[index] {
                'L' => left_map.get(current).unwrap(),
                'R' => right_map.get(current).unwrap(),
                _ => panic!("Invalid direction"),
            };
        }
        assert_eq!(verify_steps, steps);

        steps_per_starting_node.push(steps);
    }

    // Get the least common multiple
    steps_per_starting_node
        .iter()
        .fold(steps_per_starting_node[0], |acc, &x| lcm(acc, x))
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
        assert_eq!(solve_problem_1(content), 6);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 6);
    }

    #[test]
    fn test_2_4_gives_4() {
        let content = "L

AAA = (AAB, XXX)
AAB = (AAZ, XXX)
AAZ = (AAB, XXX)
BBA = (BBB, XXX)
BBB = (BBC, XXX)
BBC = (BBD, XXX)
BBD = (BBZ, XXX)
BBZ = (BBB, XXX)
"
        .to_string();
        assert_eq!(solve_problem_2(content), 4);
    }
}
