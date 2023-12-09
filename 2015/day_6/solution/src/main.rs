use std::fs;

struct Grid {
    lights: Vec<Vec<u64>>,
}

impl Grid {
    fn new() -> Self {
        Self {
            lights: vec![vec![0; 1000]; 1000],
        }
    }

    fn turn_on(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.lights[x][y] = 1;
            }
        }
    }

    fn turn_brighter(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.lights[x][y] += 1;
            }
        }
    }

    fn turn_off(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.lights[x][y] = 0;
            }
        }
    }

    fn turn_dimmer(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                if self.lights[x][y] > 0 {
                    self.lights[x][y] -= 1;
                }
            }
        }
    }

    fn toggle(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.lights[x][y] = 1 - self.lights[x][y];
            }
        }
    }

    fn turn_extra_bright(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.lights[x][y] += 2;
            }
        }
    }

    fn brightness(&self) -> u64 {
        self.lights.iter().map(|r| r.iter().sum::<u64>()).sum()
    }
}

fn solve_problem_1(input: String) -> u64 {
    let mut grid = Grid::new();
    for line in input.lines().filter(|l| !l.is_empty()) {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts[0] == "turn" {
            let from: Vec<usize> = parts[2].split(',').map(|s| s.parse().unwrap()).collect();
            let to: Vec<usize> = parts[4].split(',').map(|s| s.parse().unwrap()).collect();
            if parts[1] == "on" {
                grid.turn_on(from[0], from[1], to[0], to[1]);
            } else if parts[1] == "off" {
                grid.turn_off(from[0], from[1], to[0], to[1]);
            }
        } else {
            let from: Vec<usize> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();
            let to: Vec<usize> = parts[3].split(',').map(|s| s.parse().unwrap()).collect();
            grid.toggle(from[0], from[1], to[0], to[1]);
        }
    }
    grid.brightness()
}

fn solve_problem_2(input: String) -> u64 {
    let mut grid = Grid::new();
    for line in input.lines().filter(|l| !l.is_empty()) {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts[0] == "turn" {
            let from: Vec<usize> = parts[2].split(',').map(|s| s.parse().unwrap()).collect();
            let to: Vec<usize> = parts[4].split(',').map(|s| s.parse().unwrap()).collect();
            if parts[1] == "on" {
                grid.turn_brighter(from[0], from[1], to[0], to[1]);
            } else if parts[1] == "off" {
                grid.turn_dimmer(from[0], from[1], to[0], to[1]);
            }
        } else {
            let from: Vec<usize> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();
            let to: Vec<usize> = parts[3].split(',').map(|s| s.parse().unwrap()).collect();
            grid.turn_extra_bright(from[0], from[1], to[0], to[1]);
        }
    }
    grid.brightness()
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
        assert_eq!(solve_problem_1(content), 1_000_000 - 1_000 - 4);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 2_000_001);
    }
}
