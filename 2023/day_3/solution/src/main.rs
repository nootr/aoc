use std::fs;

#[derive(Debug)]
enum Spot {
    Nothing,
    Digit(u8),
    Symbol,
}

#[derive(Debug, Default)]
struct Map {
    width: i32,
    height: i32,
    data: Vec<Vec<Spot>>,
}

impl Map {
    fn from_string(raw_data: String) -> Self {
        let data = raw_data
            .split("\n")
            .filter(|x| !x.is_empty())
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        '.' => Spot::Nothing,
                        '0'..='9' => Spot::Digit(c.to_digit(10).unwrap() as u8),
                        _ => Spot::Symbol,
                    })
                    .collect::<Vec<Spot>>()
            })
            .collect::<Vec<Vec<Spot>>>();
        Self {
            width: data[0].len() as i32,
            height: data.len() as i32,
            data,
        }
    }

    fn find_symbol_locations(&self) -> Vec<(i32, i32)> {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, spot)| {
                    if let Spot::Symbol = spot {
                        Some((x as i32, y as i32))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<(i32, i32)>>()
    }

    fn find_adjacent_numbers(&self, x: i32, y: i32) -> Vec<u32> {
        let mut result = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                let mut number_x = (x + i) as usize;
                let number_y = (y + j) as usize;

                // Skip out of bounds
                if number_x >= self.width as usize || number_y >= self.height as usize {
                    continue;
                }

                // Skip if part of earlier parsed number
                if number_x > 0 && i >= 0 {
                    if let Spot::Digit(_) = self.data[number_y][number_x - 1] {
                        continue;
                    }
                }

                if let Spot::Digit(_) = self.data[number_y][number_x] {
                    // Find beginning of number
                    while number_x > 0 {
                        if let Spot::Digit(_) = self.data[number_y][number_x - 1] {
                            number_x -= 1;
                        } else {
                            break;
                        }
                    }

                    // Parse number
                    let mut number = 0;
                    while number_x < self.width as usize {
                        if let Spot::Digit(digit) = self.data[number_y][number_x] {
                            number = number * 10 + digit as u32;
                            number_x += 1;
                        } else {
                            break;
                        }
                    }

                    result.push(number);
                }
            }
        }
        result
    }
}

fn solve_problem_1(input: String) -> u32 {
    let map = Map::from_string(input);
    let symbol_locations = map.find_symbol_locations();
    symbol_locations
        .iter()
        .map(|(x, y)| map.find_adjacent_numbers(*x, *y).iter().sum::<u32>())
        .sum()
}

fn solve_problem_2(input: String) -> u32 {
    let map = Map::from_string(input);
    let symbol_locations = map.find_symbol_locations();
    symbol_locations
        .iter()
        .map(|(x, y)| {
            let adjacent_numbers = map.find_adjacent_numbers(*x, *y);
            if adjacent_numbers.len() > 1 {
                adjacent_numbers.iter().product()
            } else {
                0
            }
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
        let input = String::from(
            "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
",
        );
        assert_eq!(solve_problem_1(input), 4361);
    }

    #[test]
    fn problem_2_solved() {
        let input = String::from(
            "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
",
        );
        assert_eq!(solve_problem_2(input), 467835);
    }
}
