use std::collections::{HashMap, HashSet};
use std::fs;

struct Card {
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn from_line(line: &str) -> Card {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let game = parts[0];
        let data = parts[1];
        assert!(game.starts_with("Card "));

        let parts = data.split(" | ").collect::<Vec<&str>>();
        let winning_numbers_str = parts[0];
        let numbers_str = parts[1];

        let winning_numbers = winning_numbers_str
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();

        let numbers = numbers_str
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();

        Card {
            winning_numbers,
            numbers,
        }
    }

    fn wins(&self) -> usize {
        self.numbers
            .iter()
            .filter(|x| self.winning_numbers.contains(x))
            .count()
    }

    fn score(&self) -> u32 {
        match self.wins() {
            0 => 0_u32,
            n => 2_i32.pow((n - 1) as u32) as u32,
        }
    }
}

fn solve_problem_1(input: String) -> u32 {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| Card::from_line(line).score())
        .sum()
}

fn solve_problem_2(input: String) -> u32 {
    let cards = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(Card::from_line)
        .collect::<Vec<Card>>();

    let mut copies = HashMap::new();

    let mut total_cards = 0;

    for (i, card) in cards.iter().enumerate() {
        let number_of_cards = copies.get(&i).unwrap_or(&0) + 1;

        let wins = card.wins();

        total_cards += number_of_cards;
        for j in 0..wins {
            let cards_to_add = copies.get(&(i + j + 1)).unwrap_or(&0) + number_of_cards;
            copies.insert(i + j + 1, cards_to_add);
        }
    }
    total_cards
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
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
",
        );
        assert_eq!(solve_problem_1(input), 13);
    }

    #[test]
    fn problem_2_solved() {
        let input = String::from(
            "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
",
        );
        assert_eq!(solve_problem_2(input), 30);
    }
}
