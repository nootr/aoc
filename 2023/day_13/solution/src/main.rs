use std::fs;

#[derive(Debug)]
struct Grid {
    row_hashes: Vec<u64>,
    col_hashes: Vec<u64>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut row_hashes = Vec::new();
        let mut col_hashes = Vec::new();

        for (y, line) in input.lines().enumerate() {
            row_hashes.push(0);
            for (x, c) in line.chars().enumerate() {
                if col_hashes.len() <= x {
                    col_hashes.push(0);
                }
                match c {
                    '#' => {
                        row_hashes[y] += 1 << x;
                        col_hashes[x] += 1 << y;
                    }
                    '.' => {}
                    c => panic!("Unexpected character: {}", c),
                }
            }
        }

        let width = col_hashes.len();
        let height = row_hashes.len();

        Self {
            row_hashes,
            col_hashes,
            width,
            height,
        }
    }

    fn check_symmetry(array: &[u64], size: usize, with_smudge: bool) -> Option<usize> {
        for i in 0..(size - 1) {
            let mut encountered_smudge = false;
            'outer: for di in 1..size {
                let a = i as isize + 1 - di as isize;
                let b = i + di;
                if a < 0 || b >= size {
                    if !with_smudge || encountered_smudge {
                        return Some(i + 1);
                    }
                    break;
                }
                if with_smudge {
                    let cmp = array[a as usize] ^ array[b];
                    let mut smudges = 0;
                    for b in 0..64 {
                        if cmp & (1 << b) != 0 {
                            smudges += 1;
                            if smudges > 1 {
                                break 'outer;
                            }
                        }
                    }
                    if smudges == 1 {
                        if encountered_smudge {
                            break;
                        }
                        encountered_smudge = true;
                    }
                } else if array[a as usize] != array[b] {
                    break;
                }
            }
        }
        None
    }

    fn horizontal_mirror(&self) -> usize {
        Self::check_symmetry(&self.row_hashes, self.height, false).unwrap_or(0)
    }

    fn vertical_mirror(&self) -> usize {
        Self::check_symmetry(&self.col_hashes, self.width, false).unwrap_or(0)
    }

    fn horizontal_smudged_mirror(&self) -> usize {
        Self::check_symmetry(&self.row_hashes, self.height, true).unwrap_or(0)
    }

    fn vertical_smudged_mirror(&self) -> usize {
        Self::check_symmetry(&self.col_hashes, self.width, true).unwrap_or(0)
    }
}

fn solve_problem_1(input: String) -> usize {
    input
        .split("\n\n")
        .map(Grid::new)
        .map(|g| {
            let v = g.vertical_mirror();
            let h = g.horizontal_mirror();
            assert!(v == 0 || h == 0);
            assert!(v != 0 || h != 0);
            v + 100 * h
        })
        .sum()
}

fn solve_problem_2(input: String) -> usize {
    input
        .split("\n\n")
        .map(Grid::new)
        .map(|g| {
            let v = g.vertical_smudged_mirror();
            let h = g.horizontal_smudged_mirror();
            assert!(v == 0 || h == 0);
            assert!(v != 0 || h != 0);
            v + 100 * h
        })
        .sum()
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
        assert_eq!(solve_problem_1(content), 405);
    }

    #[test]
    fn problem_1_custom() {
        let content = "###.##......##.##
#.#.###....###.#.
..#.###....###.#.
###.##......##.##
.#####.####.#####
..#.##..##..##.#.
.###..#....#..###
..##...#..#...##.
...##.######.##..
..#.....##.....#.
.....#.####.#...."
            .to_string();
        assert_eq!(solve_problem_1(content), 9);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 400);
    }

    #[test]
    fn problem_2_custom() {
        let content = "###.#.###
.#..####.
.##.#.#..
.##.#.#..
.#..####.
###.#.###
.#.#.#.##
######.##
#.#....#.
#.#....#.
######.##
.#.#.#.##
###.#.###
.#..####.
.##.#...."
            .to_string();
        assert_eq!(solve_problem_2(content), 900);
    }
}
