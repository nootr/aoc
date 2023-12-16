use rayon::prelude::*;
use std::collections::HashSet;
use std::{fmt, fs};

#[derive(Debug, Clone, Copy)]
enum TravelDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
enum TravelOrientation {
    Horizontal,
    Vertical,
}

impl From<TravelDirection> for TravelOrientation {
    fn from(direction: TravelDirection) -> Self {
        match direction {
            TravelDirection::North | TravelDirection::South => TravelOrientation::Vertical,
            TravelDirection::East | TravelDirection::West => TravelOrientation::Horizontal,
        }
    }
}

#[derive(Debug, PartialEq, Hash, Copy, Clone)]
enum Dot {
    MirrorTopLeftBottomRight,
    MirrorTopRightBottomLeft,
    SplitterHorizontal,
    SplitterVertical,
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    dots: Vec<Vec<Option<Dot>>>,
    dots_energized_horizontal: HashSet<(usize, usize)>,
    dots_energized_vertical: HashSet<(usize, usize)>,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.dots
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(x, dot)| match dot {
                            None if self.dots_energized_horizontal.contains(&(x, y))
                                || self.dots_energized_vertical.contains(&(x, y)) =>
                            {
                                "X"
                            }
                            None => ".",
                            Some(Dot::MirrorTopLeftBottomRight) => "\\",
                            Some(Dot::MirrorTopRightBottomLeft) => "/",
                            Some(Dot::SplitterHorizontal) => "-",
                            Some(Dot::SplitterVertical) => "|",
                        })
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Grid {
    fn new(input: String) -> Self {
        let dots: Vec<Vec<Option<Dot>>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => None,
                        '\\' => Some(Dot::MirrorTopLeftBottomRight),
                        '/' => Some(Dot::MirrorTopRightBottomLeft),
                        '-' => Some(Dot::SplitterHorizontal),
                        '|' => Some(Dot::SplitterVertical),
                        c => panic!("Unexpected character: {}", c),
                    })
                    .collect()
            })
            .collect();
        Self {
            width: dots[0].len(),
            height: dots.len(),
            dots,
            dots_energized_horizontal: HashSet::new(),
            dots_energized_vertical: HashSet::new(),
        }
    }

    fn travel_beam(&mut self, place: (usize, usize), direction: TravelDirection) {
        // println!("=====");
        // println!("{:?} {:?}", place, direction);
        // println!("{:?}", self);

        let orientation = TravelOrientation::from(direction);
        let check_if_traveled_before = if let Some(dot) = self.dots[place.1][place.0] {
            if dot == Dot::MirrorTopLeftBottomRight || dot == Dot::MirrorTopRightBottomLeft {
                // I'm too lazy to write the logic for this (TODO?)
                false
            } else {
                true
            }
        } else {
            true
        };
        if orientation == TravelOrientation::Horizontal {
            if check_if_traveled_before && self.dots_energized_horizontal.contains(&place) {
                return;
            }
            self.dots_energized_horizontal.insert(place);
        }
        if orientation == TravelOrientation::Vertical {
            if check_if_traveled_before && self.dots_energized_vertical.contains(&place) {
                return;
            }
            self.dots_energized_vertical.insert(place);
        }
        match (direction, self.dots[place.1][place.0]) {
            (TravelDirection::North, None)
            | (TravelDirection::North, Some(Dot::SplitterVertical))
            | (TravelDirection::East, Some(Dot::MirrorTopRightBottomLeft))
            | (TravelDirection::West, Some(Dot::MirrorTopLeftBottomRight)) => {
                if place.1 > 0 {
                    self.travel_beam((place.0, place.1 - 1), TravelDirection::North);
                }
            }
            (TravelDirection::South, None)
            | (TravelDirection::South, Some(Dot::SplitterVertical))
            | (TravelDirection::West, Some(Dot::MirrorTopRightBottomLeft))
            | (TravelDirection::East, Some(Dot::MirrorTopLeftBottomRight)) => {
                if place.1 < self.height - 1 {
                    self.travel_beam((place.0, place.1 + 1), TravelDirection::South);
                }
            }
            (TravelDirection::East, None)
            | (TravelDirection::East, Some(Dot::SplitterHorizontal))
            | (TravelDirection::North, Some(Dot::MirrorTopRightBottomLeft))
            | (TravelDirection::South, Some(Dot::MirrorTopLeftBottomRight)) => {
                if place.0 < self.width - 1 {
                    self.travel_beam((place.0 + 1, place.1), TravelDirection::East);
                }
            }
            (TravelDirection::West, None)
            | (TravelDirection::West, Some(Dot::SplitterHorizontal))
            | (TravelDirection::South, Some(Dot::MirrorTopRightBottomLeft))
            | (TravelDirection::North, Some(Dot::MirrorTopLeftBottomRight)) => {
                if place.0 > 0 {
                    self.travel_beam((place.0 - 1, place.1), TravelDirection::West);
                }
            }
            (TravelDirection::North, Some(Dot::SplitterHorizontal))
            | (TravelDirection::South, Some(Dot::SplitterHorizontal)) => {
                if place.0 > 0 {
                    self.travel_beam((place.0 - 1, place.1), TravelDirection::West);
                }
                if place.0 < self.width - 1 {
                    self.travel_beam((place.0 + 1, place.1), TravelDirection::East);
                }
            }
            (TravelDirection::East, Some(Dot::SplitterVertical))
            | (TravelDirection::West, Some(Dot::SplitterVertical)) => {
                if place.1 > 0 {
                    self.travel_beam((place.0, place.1 - 1), TravelDirection::North);
                }
                if place.1 < self.height - 1 {
                    self.travel_beam((place.0, place.1 + 1), TravelDirection::South);
                }
            }
        }
    }

    fn energized(&self) -> HashSet<(usize, usize)> {
        self.dots_energized_horizontal
            .union(&self.dots_energized_vertical)
            .copied()
            .collect::<HashSet<(usize, usize)>>()
    }
}

fn solve_problem_1(input: String) -> u64 {
    let mut grid = Grid::new(input);
    grid.travel_beam((0, 0), TravelDirection::East);
    grid.energized().len() as u64
}

fn solve_problem_2(input: String) -> u64 {
    let grid = Grid::new(input);

    let mut cases = vec![];
    for x in 0..grid.width {
        for y in 0..grid.height {
            for direction in [
                TravelDirection::North,
                TravelDirection::East,
                TravelDirection::South,
                TravelDirection::West,
            ] {
                cases.push((x, y, direction));
            }
        }
    }
    println!("Cases: {}", cases.len());

    cases
        .par_iter()
        .map(|(x, y, direction)| {
            let mut grid = grid.clone();
            grid.travel_beam((*x, *y), *direction);
            grid.energized().len() as u64
        })
        .max()
        .unwrap()
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
        assert_eq!(solve_problem_1(content), 46);
    }

    #[test]
    fn issue_is_fixed() {
        let content = fs::read_to_string("../input")
            .expect("Should have been able to read the file");
        let mut grid = Grid::new(content);
        grid.travel_beam((49, 40), TravelDirection::West);
        assert_ne!(grid.energized().len(), 7139);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 51);
    }
}
