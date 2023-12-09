use std::collections::HashSet;
use std::fs;

fn solve_problem_1(input: String) -> usize {
    let mut houses = HashSet::new();

    let mut x = 0;
    let mut y = 0;
    for direction in input.chars() {
        houses.insert((x, y));
        match direction {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => {}
        }
    }

    houses.len()
}

fn solve_problem_2(input: String) -> usize {
    let mut houses = HashSet::new();

    let mut x_a = 0;
    let mut y_a = 0;
    let mut x_b = 0;
    let mut y_b = 0;
    for (i, direction) in input.chars().enumerate() {
        if i % 2 == 0 {
            houses.insert((x_a, y_a));
            match direction {
                '^' => y_a += 1,
                'v' => y_a -= 1,
                '>' => x_a += 1,
                '<' => x_a -= 1,
                _ => {}
            }
        } else {
            houses.insert((x_b, y_b));
            match direction {
                '^' => y_b += 1,
                'v' => y_b -= 1,
                '>' => x_b += 1,
                '<' => x_b -= 1,
                _ => {}
            }
        }
    }

    houses.len()
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
        assert_eq!(solve_problem_1(content), 4);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 3);
    }
}
