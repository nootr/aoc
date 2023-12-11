use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    location: (usize, usize),
}

#[derive(Debug, Default)]
struct Universe {
    galaxies: Vec<Galaxy>,
    location_map: Vec<Vec<(usize, usize)>>,
}

impl Universe {
    fn new(input: String, expansion: u64) -> Self {
        let spots: Vec<Vec<Option<Galaxy>>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, g)| match g {
                        '.' => None,
                        '#' => Some(Galaxy { location: (x, y) }),
                        _ => panic!("Unexpected character: {}", g),
                    })
                    .collect()
            })
            .collect();

        let height = spots.len();
        let width = spots[0].len();
        let galaxies: Vec<Galaxy> = spots.into_iter().flatten().flatten().collect();
        let location_map = Self::calculate_location_map(&galaxies, width, height, expansion);

        Self {
            galaxies,
            location_map,
        }
    }

    fn calculate_location_map(
        galaxies: &[Galaxy],
        width: usize,
        height: usize,
        expansion: u64,
    ) -> Vec<Vec<(usize, usize)>> {
        // Calculate location map to account for expanding of the universe
        let mut location_map = vec![];
        let mut non_empty_rows: HashSet<usize> = HashSet::new();
        let mut non_empty_cols: HashSet<usize> = HashSet::new();

        for galaxy in galaxies.iter() {
            non_empty_rows.insert(galaxy.location.0);
            non_empty_cols.insert(galaxy.location.1);
        }

        let mut dx = 0;
        for x in 0..width {
            let mut row = vec![];
            if !non_empty_rows.contains(&x) {
                dx += expansion as usize;
            } else {
                let mut dy = 0;
                for y in 0..height {
                    if !non_empty_cols.contains(&y) {
                        dy += expansion as usize;
                    }
                    row.push(((x + dx), (y + dy)));
                }
            }
            location_map.push(row);
        }
        location_map
    }

    fn galaxy_pairs(&self) -> Vec<(&Galaxy, &Galaxy)> {
        self.galaxies
            .iter()
            .enumerate()
            .flat_map(|(i, a)| self.galaxies[i + 1..].iter().map(move |b| (a, b)))
            .collect()
    }

    fn get_expanded_location(&self, pos: (usize, usize)) -> (usize, usize) {
        self.location_map[pos.0][pos.1]
    }

    fn distance(&self, a: &Galaxy, b: &Galaxy) -> u64 {
        let position_a = self.get_expanded_location(a.location);
        let position_b = self.get_expanded_location(b.location);
        ((position_a.0 as i64 - position_b.0 as i64).abs()
            + (position_a.1 as i64 - position_b.1 as i64).abs()) as u64
    }
}

fn solve_problem(input: String, expansion: u64) -> u64 {
    let universe = Universe::new(input, expansion);
    universe
        .galaxy_pairs()
        .iter()
        .map(|(a, b)| universe.distance(a, b))
        .sum()
}

fn main() {
    let content = fs::read_to_string("../input").expect("Should have been able to read the file");

    let solution = solve_problem(content.clone(), 1);
    println!("Solution part 1: {}", solution);

    let solution = solve_problem(content, 1_000_000 - 1);
    println!("Solution part 2: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_1_solved() {
        let content = fs::read_to_string("../input-example-1")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem(content, 1), 374);
    }

    #[test]
    fn galaxy_pairs_are_correct_length() {
        let content = fs::read_to_string("../input-example-1")
            .expect("Should have been able to read the file");
        let universe = Universe::new(content, 1);
        assert_eq!(universe.galaxy_pairs().len(), 36);
    }

    #[test]
    fn distances_are_correct_length() {
        let content = fs::read_to_string("../input-example-1")
            .expect("Should have been able to read the file");
        let universe = Universe::new(content, 1);
        let galaxy_a = universe.galaxies[0];
        let galaxy_b = universe.galaxies[1];
        assert_eq!(universe.distance(&galaxy_a, &galaxy_b), 6);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem(content, 100 - 1), 8410);
    }
}
