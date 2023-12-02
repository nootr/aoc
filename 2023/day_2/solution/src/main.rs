use std::cmp::max;
use std::fs;

#[derive(Debug)]
struct Game {
    id: u32,
    min_green: u32,
    min_red: u32,
    min_blue: u32,
}

impl Game {
    fn from_line(line: &str) -> Option<Self> {
        if !line.starts_with("Game ") {
            return None;
        }
        let line = &line[5..];
        let parts = line.split(": ").collect::<Vec<&str>>();
        let id = parts[0].parse::<u32>().unwrap();
        let line = parts[1];

        let mut min_green = 0;
        let mut min_blue = 0;
        let mut min_red = 0;

        for case in line.split(";") {
            for color_case in case.split(",") {
                let color_case = color_case.trim();
                let parts = color_case.split(" ").collect::<Vec<&str>>();
                let count = parts[0].parse::<u32>().unwrap();
                let color = parts[1];
                match color {
                    "red" => {
                        min_red = max(min_red, count);
                    }
                    "green" => {
                        min_green = max(min_green, count);
                    }
                    "blue" => {
                        min_blue = max(min_blue, count);
                    }
                    c => panic!("Unknown color: {}", c),
                }
            }
        }

        Some(Self {
            id,
            min_green,
            min_red,
            min_blue,
        })
    }

    fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        self.min_red <= red && self.min_green <= green && self.min_blue <= blue
    }

    fn power(&self) -> u32 {
        self.min_red * self.min_green * self.min_blue
    }
}

fn solve(input: String, red: u32, green: u32, blue: u32) -> u32 {
    input
        .split("\n")
        .map(|line| Game::from_line(line))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .filter(|x| x.is_possible(red, green, blue))
        .map(|x| x.id)
        .sum::<u32>()
}

fn solve_part_2(input: String) -> u32 {
    input
        .split("\n")
        .map(|line| Game::from_line(line))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .map(|x| x.power())
        .sum::<u32>()
}

fn main() {
    let content = fs::read_to_string("../input").expect("Should have been able to read the file");

    let solution = solve(content.clone(), 12, 13, 14);
    println!("Solution part 1: {}", solution);

    let solution = solve_part_2(content);
    println!("Solution part 2: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_problem_works() {
        let input = String::from(
            "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
",
        );
        assert_eq!(solve(input, 12, 13, 14), 8);
    }

    #[test]
    fn part_2_example_problem_works() {
        let input = String::from(
            "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
",
        );
        assert_eq!(solve_part_2(input), 2286);
    }
}
