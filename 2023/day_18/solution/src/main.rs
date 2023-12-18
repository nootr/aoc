use std::fs;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Step {
    direction: Direction,
    length: u64,
}

impl Step {
    fn from_line(line: &str) -> Self {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let direction = match parts[0] {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            x => panic!("Unknown direction: {}", x),
        };
        let length = parts[1].parse::<u64>().unwrap();
        Self { direction, length }
    }

    fn from_color(line: &str) -> Self {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let hex = &parts[2][2..parts[2].len() - 2];
        let length = u64::from_str_radix(hex, 16).unwrap();
        let direction = match line.chars().nth(line.len() - 2).unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            x => panic!("Unknown direction: {:?}", x),
        };
        Self { direction, length }
    }
}

struct Grid {
    steps: Vec<Step>,
}

impl Grid {
    fn new(steps: Vec<Step>) -> Self {
        Self { steps }
    }

    /// Use shoelace-like algorithm to find area
    fn area(&self) -> u64 {
        let mut y = 0;
        let area_minus_one: u64 = self
            .steps
            .iter()
            .map(|step| match step.direction {
                Direction::Right => -(step.length as i64) * y,
                Direction::Left => (step.length as i64) * (y + 1),
                Direction::Up => {
                    y -= step.length as i64;
                    0
                }
                Direction::Down => {
                    y += step.length as i64;
                    step.length as i64
                }
            })
            .sum::<i64>()
            .abs()
            .try_into()
            .unwrap();

        area_minus_one + 1 // WTF??!?!?! Why does this work?
    }
}

fn solve_problem_1(input: String) -> u64 {
    let steps: Vec<Step> = input.lines().map(Step::from_line).collect();
    let grid = Grid::new(steps);
    grid.area()
}

fn solve_problem_2(input: String) -> u64 {
    let steps: Vec<Step> = input.lines().map(Step::from_color).collect();
    let grid = Grid::new(steps);
    grid.area()
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
        assert_eq!(solve_problem_1(content), 62);
    }

    #[test]
    fn problem_1_custom() {
        // ... ... 6
        // . ... . 7
        // .     . 7
        // ....... 7
        let content = "R 2 #000000
D 1 #000000
R 2 #000000
U 1 #000000
R 2 #000000
D 3 #000000
L 6 #000000
U 3 #000000"
            .to_string();
        assert_eq!(solve_problem_1(content), 3 * 7 + 6);
    }

    #[test]
    fn problem_1_custom_2() {
        // ......
        // . .. .
        // .    .
        // ......
        let content = "R 2 #000000
D 1 #000000
R 1 #000000
U 1 #000000
R 2 #000000
D 3 #000000
L 5 #000000
U 3 #000000"
            .to_string();
        assert_eq!(solve_problem_1(content), 4 * 6);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 952408144115);
    }
}
