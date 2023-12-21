use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Dot {
    Empty,
    Rock,
}

struct Grid {
    height: usize,
    width: usize,
    dots: Vec<Vec<Dot>>,
    starting_position: (usize, usize),
}

impl Grid {
    fn from_input(input: String) -> Self {
        let mut starting_position = None;
        let dots: Vec<Vec<Dot>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Dot::Empty,
                        '#' => Dot::Rock,
                        'S' => {
                            starting_position = Some((x, y));
                            Dot::Empty
                        }
                        c => panic!("Invalid input: {}", c),
                    })
                    .collect()
            })
            .collect();
        Self {
            height: dots.len(),
            width: dots[0].len(),
            dots,
            starting_position: starting_position.unwrap(),
        }
    }

    fn possibilities_after_steps(&self, steps: i64) -> u64 {
        // NOTE: % is not modulo, but remainder. Use rem_euclid() instead:
        assert_ne!((-1i64) % 5, 4);
        assert_eq!((-1i64).rem_euclid(5), 4);

        let mut queue = vec![(
            (
                self.starting_position.0 as isize,
                self.starting_position.1 as isize,
            ),
            0_i64,
        )];
        let mut visited = HashSet::new();
        let mut cache = HashMap::new();
        while !queue.is_empty() {
            let ((x, y), steps_done) = queue.remove(0);
            if steps_done > steps || visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let (nx, ny) = (x as isize + dx, y as isize + dy);
                if self.dots[(ny.rem_euclid(self.height as isize)) as usize]
                    [(nx.rem_euclid(self.width as isize)) as usize]
                    == Dot::Rock
                {
                    continue;
                }

                queue.push(((nx, ny), steps_done + 1));
                cache.insert((nx, ny), steps_done + 1);
            }
            if steps_done % 2 != 0 {
                continue;
            }
            for (dx, dy) in [(1, 1), (-1, -1), (1, -1), (-1, 1)] {
                let earlier_steps_done = match cache
                    .get(&(x - dx * self.width as isize, y - dy * self.height as isize))
                {
                    Some(x) => *x,
                    None => continue,
                };
                let even_earlier_steps_done = match cache.get(&(
                    x - 2 * dx * self.width as isize,
                    y - 2 * dy * self.height as isize,
                )) {
                    Some(x) => *x,
                    None => continue,
                };

                let delta = steps_done - earlier_steps_done;
                let delta_test = earlier_steps_done - even_earlier_steps_done;
                if delta != 0 && delta == delta_test {
                    let mut steps_done = steps_done;
                    let mut nx = x;
                    let mut ny = y;
                    while steps_done < steps {
                        visited.insert((nx, ny));
                        steps_done += delta;
                        nx += dx * self.width as isize;
                        ny += dy * self.height as isize;
                    }
                }
            }
        }
        visited
            .iter()
            .filter(|(x, y)| {
                ((x + y) as i64 - (self.starting_position.0 + self.starting_position.1) as i64)
                    .rem_euclid(2)
                    == 0
            })
            .count() as u64
    }
}

fn solve_problem(input: String, steps: i64) -> u64 {
    let grid = Grid::from_input(input);
    grid.possibilities_after_steps(steps)
}

fn main() {
    let content = fs::read_to_string("../input").expect("Should have been able to read the file");

    let solution = solve_problem(content.clone(), 64);
    println!("Solution part 1: {}", solution);

    let solution = solve_problem(content, 26501365);
    println!("Solution part 2: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_1_solved() {
        let content = fs::read_to_string("../input-example-1")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem(content, 6), 16);
    }

    #[test]
    fn problem_2_solved_a() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem(content, 6), 16);
    }

    #[test]
    fn problem_2_solved_b() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem(content, 10), 50);
    }

    #[test]
    fn problem_2_solved_c() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem(content, 50), 1594);
    }

    #[test]
    fn problem_2_solved_d() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem(content, 100), 6536);
    }

    #[test]
    fn problem_2_solved_e() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem(content, 500), 167004);
    }

    #[test]
    fn problem_2_solved_f() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem(content, 1000), 668697);
    }

    #[test]
    fn problem_2_solved_g() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem(content, 5000), 16733044);
    }
}
