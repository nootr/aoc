use std::collections::HashMap;
use std::fs;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug)]
struct Node {
    heat_loss: u8,
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    nodes: Vec<Vec<Node>>,
}

impl Grid {
    fn new(input: String) -> Grid {
        let nodes: Vec<Vec<Node>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Node {
                        heat_loss: c.to_digit(10).unwrap() as u8,
                    })
                    .collect()
            })
            .collect();

        Grid {
            width: nodes[0].len(),
            height: nodes.len(),
            nodes,
        }
    }

    fn node(&self, x: usize, y: usize) -> &Node {
        &self.nodes[y][x]
    }

    /// Find the path with the lowest heat loss and not more than three consecutive moves in the
    /// same direction and return the cumulative heat loss.
    fn best_path_heat_loss(&self, direction_moves_min: u8, direction_moves_max: u8) -> u64 {
        let mut unvisited: HashMap<((usize, usize), Direction, u8), Option<u64>> =
            HashMap::from_iter(self.nodes.iter().enumerate().flat_map(|(y, row)| {
                [
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ]
                .iter()
                .flat_map(move |direction| {
                    (1..=direction_moves_max).flat_map(move |i| {
                        row.iter()
                            .enumerate()
                            .map(move |(x, _)| (((x, y), *direction, i), None))
                    })
                })
            }));
        unvisited.insert(((0, 0), Direction::East, 0), Some(0));

        loop {
            // Find unvisited node with lowest heat loss
            let mut current = (0, 0);
            let mut cumulative_heat_loss = u64::MAX;
            let mut last_direction = Direction::East;
            let mut last_direction_moves = 0;
            for ((pos, direction, direction_moves), heat_loss) in &unvisited {
                if heat_loss.is_some() && heat_loss.unwrap() < cumulative_heat_loss {
                    current = *pos;
                    cumulative_heat_loss = heat_loss.unwrap();
                    last_direction = *direction;
                    last_direction_moves = *direction_moves;
                }
            }

            // Check if a next node is available
            if cumulative_heat_loss == u64::MAX {
                panic!("Could not find route");
            }

            // Check if goal is reached
            if current.0 == self.width - 1 && current.1 == self.height - 1 {
                if last_direction_moves >= direction_moves_min {
                    return cumulative_heat_loss;
                }
                unvisited.remove(&(current, last_direction, last_direction_moves));
                continue;
            }
            unvisited.remove(&(current, last_direction, last_direction_moves));

            // Find possible neighbours
            let neighbours: Vec<((usize, usize), Direction)> = [
                (
                    (current.0 as isize + 1, current.1 as isize),
                    Direction::East,
                ),
                (
                    (current.0 as isize - 1, current.1 as isize),
                    Direction::West,
                ),
                (
                    (current.0 as isize, current.1 as isize + 1),
                    Direction::South,
                ),
                (
                    (current.0 as isize, current.1 as isize - 1),
                    Direction::North,
                ),
            ]
            .iter()
            .filter(|((x, y), direction)| {
                *x < self.width as isize
                    && *y < self.height as isize
                    && *x >= 0
                    && *y >= 0
                    && (*direction != last_direction || last_direction_moves < direction_moves_max)
                    && (*direction == last_direction
                        || last_direction_moves + 1 > direction_moves_min)
                    && (last_direction != direction.opposite())
            })
            .map(|((x, y), d)| ((*x as usize, *y as usize), *d))
            .collect();

            // Update the cumulative heat loss of the neighbours
            for (neighbour, direction) in &neighbours {
                let neighbour_heat_loss =
                    cumulative_heat_loss + self.node(neighbour.0, neighbour.1).heat_loss as u64;
                let neighbour_moves = if last_direction == *direction {
                    last_direction_moves + 1
                } else {
                    1
                };
                let key = (*neighbour, *direction, neighbour_moves);
                if unvisited.contains_key(&key) {
                    let neighbour_current_heat_loss = *unvisited.get(&key).unwrap();

                    if neighbour_current_heat_loss.is_none()
                        || neighbour_heat_loss < neighbour_current_heat_loss.unwrap()
                    {
                        unvisited.insert(key, Some(neighbour_heat_loss));
                    }
                }
            }
        }
    }
}

fn solve_problem_1(input: String) -> u64 {
    let grid = Grid::new(input);
    grid.best_path_heat_loss(0, 3)
}

fn solve_problem_2(input: String) -> u64 {
    let grid = Grid::new(input);
    grid.best_path_heat_loss(4, 10)
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
        assert_eq!(solve_problem_1(content), 102);
    }

    #[test]
    fn problem_2_example_a() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 94);
    }

    #[test]
    fn problem_2_example_b() {
        let content = fs::read_to_string("../input-example-3")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 71);
    }
}
