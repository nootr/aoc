use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl From<(isize, isize)> for Direction {
    fn from(coordinates: (isize, isize)) -> Self {
        match coordinates {
            (-1, 0) => Self::W,
            (1, 0) => Self::E,
            (0, -1) => Self::N,
            (0, 1) => Self::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum NodeShape {
    NS,
    EW,
    NW,
    SW,
    NE,
    SE,
}

impl From<(Direction, Direction)> for NodeShape {
    fn from(directions: (Direction, Direction)) -> Self {
        match directions {
            (Direction::N, Direction::S) | (Direction::S, Direction::N) => Self::NS,
            (Direction::E, Direction::W) | (Direction::W, Direction::E) => Self::EW,
            (Direction::N, Direction::W) | (Direction::W, Direction::N) => Self::NW,
            (Direction::S, Direction::W) | (Direction::W, Direction::S) => Self::SW,
            (Direction::N, Direction::E) | (Direction::E, Direction::N) => Self::NE,
            (Direction::S, Direction::E) | (Direction::E, Direction::S) => Self::SE,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    neighbours: [(isize, isize); 2],
    shape: NodeShape,
}

#[derive(Debug)]
struct Grid {
    nodes: Vec<Vec<Option<Node>>>,
    start: (isize, isize),
}

impl Grid {
    fn new(input: String) -> Self {
        let mut start = None;
        let mut nodes: Vec<Vec<Option<Node>>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                let y = y as isize;
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let x = x as isize;
                        match c {
                            '|' => Some(Node {
                                neighbours: [(x, y - 1), (x, y + 1)],
                                shape: NodeShape::NS,
                            }),
                            '-' => Some(Node {
                                neighbours: [(x - 1, y), (x + 1, y)],
                                shape: NodeShape::EW,
                            }),
                            'L' => Some(Node {
                                neighbours: [(x, y - 1), (x + 1, y)],
                                shape: NodeShape::NE,
                            }),
                            'J' => Some(Node {
                                neighbours: [(x - 1, y), (x, y - 1)],
                                shape: NodeShape::NW,
                            }),
                            '7' => Some(Node {
                                neighbours: [(x - 1, y), (x, y + 1)],
                                shape: NodeShape::SW,
                            }),
                            'F' => Some(Node {
                                neighbours: [(x + 1, y), (x, y + 1)],
                                shape: NodeShape::SE,
                            }),
                            'S' => {
                                start = Some((x, y));
                                None
                            }
                            '.' => None,
                            _ => {
                                panic!("Unknown node: {}", c)
                            }
                        }
                    })
                    .collect()
            })
            .collect();

        let width = nodes[0].len();
        let height = nodes.len();

        // Set starting point
        let start = start.expect("Grid should have starting point");
        let mut starting_node_neighbours = vec![];
        let mut directions = vec![];
        for dx in -1..=1 {
            for dy in -1..=1 {
                let neighbour = (start.0 + dx, start.1 + dy);
                if neighbour.0 < 0
                    || neighbour.1 < 0
                    || neighbour.0 >= width as isize
                    || neighbour.1 >= height as isize
                {
                    continue;
                }
                if let Some(node) = nodes[neighbour.1 as usize][neighbour.0 as usize] {
                    if node.neighbours.contains(&start) {
                        starting_node_neighbours.push(neighbour);
                        directions.push((dx, dy).into());
                    }
                }
            }
        }
        nodes[start.1 as usize][start.0 as usize] = Some(Node {
            neighbours: [starting_node_neighbours[0], starting_node_neighbours[1]],
            shape: (directions[0], directions[1]).into(),
        });

        Self { nodes, start }
    }

    fn node(&self, pos: (isize, isize)) -> Option<Node> {
        self.nodes[pos.1 as usize][pos.0 as usize]
    }

    fn main_loop(&self) -> HashSet<(isize, isize)> {
        let mut last = self.start;
        let mut current = self.node(self.start).unwrap().neighbours[0];
        let mut main_loop = HashSet::new();

        main_loop.insert(current);
        loop {
            let next = *self
                .node(current)
                .unwrap()
                .neighbours
                .iter()
                .find(|pos| **pos != last)
                .unwrap();
            last = current;
            current = next;
            main_loop.insert(current);

            if current == self.start {
                break;
            }
        }

        main_loop
    }
}

fn solve_problem_1(input: String) -> usize {
    let grid = Grid::new(input);
    grid.main_loop().len() / 2
}

fn solve_problem_2(input: String) -> u64 {
    let grid = Grid::new(input);
    let main_loop = grid.main_loop();

    let mut dots_inside = 0;
    for (y, row) in grid.nodes.iter().enumerate() {
        let y = y as isize;
        let mut is_inside = false;
        let mut loop_came_from = None;
        for (x, _) in row.iter().enumerate() {
            let x = x as isize;
            if main_loop.contains(&(x, y)) {
                let node = grid.node((x, y)).unwrap();
                match node.shape {
                    NodeShape::NS => {
                        is_inside = !is_inside;
                    }
                    NodeShape::EW => {}
                    NodeShape::NW => {
                        is_inside ^=
                            loop_came_from.expect("Main loop should be complete") == Direction::S;
                        loop_came_from = None;
                    }
                    NodeShape::SW => {
                        is_inside ^=
                            loop_came_from.expect("Main loop should be complete") == Direction::N;
                        loop_came_from = None;
                    }
                    NodeShape::NE => {
                        loop_came_from = Some(Direction::N);
                    }
                    NodeShape::SE => {
                        loop_came_from = Some(Direction::S);
                    }
                }
            } else if is_inside {
                dots_inside += 1;
            }
        }
    }
    dots_inside
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
        assert_eq!(solve_problem_1(content), 8);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 10);
    }
}
