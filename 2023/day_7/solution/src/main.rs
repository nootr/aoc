use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

static WILDCARD: u32 = 1;

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
}

impl Hand {
    /// Returns the type of hand, 1 being the lowest (high card)
    fn hand_type(&self) -> HandType {
        let mut groups: [u32; 15] = [0; 15];
        let mut seen_cards = HashSet::new();
        let mut number_of_wildcards = 0;

        for card in &self.cards {
            if *card == WILDCARD {
                number_of_wildcards += 1;
            } else {
                let index = *card as usize;
                groups[index] += 1;
                seen_cards.insert(index);
            }
        }

        // Append number_of_wildcards to the cards
        if seen_cards.is_empty() {
            // All cards are wildcards
            return HandType::FiveOfAKind;
        }
        let most_occuring_index = seen_cards
            .iter()
            .max_by(|a, b| groups[**a].cmp(&groups[**b]))
            .unwrap();
        groups[*most_occuring_index] += number_of_wildcards;
        groups[WILDCARD as usize] = groups[*most_occuring_index];

        let hand_type = match seen_cards.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 if groups[self.cards[0] as usize] == 1 && groups[self.cards[1] as usize] == 2 => {
                HandType::TwoPair
            }
            3 if groups[self.cards[0] as usize] == 2 => HandType::TwoPair,
            3 => HandType::ThreeOfAKind,
            2 if groups[self.cards[0] as usize] == 2 => HandType::FullHouse,
            2 if groups[self.cards[0] as usize] == 3 => HandType::FullHouse,
            2 => HandType::FourOfAKind,
            _ => HandType::FiveOfAKind,
        };

        assert!(!(number_of_wildcards > 0 && hand_type == HandType::HighCard));
        assert!(!(number_of_wildcards > 0 && hand_type == HandType::TwoPair));
        assert!(!(number_of_wildcards == 4 && hand_type != HandType::FiveOfAKind));

        hand_type
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type = self.hand_type();
        let other_hand_type = other.hand_type();

        if hand_type < other_hand_type {
            Ordering::Greater
        } else if hand_type > other_hand_type {
            Ordering::Less
        } else {
            for i in 0..self.cards.len() {
                match self.cards[i].cmp(&other.cards[i]) {
                    Ordering::Equal => continue,
                    ord => return ord,
                }
            }
            panic!("There should be no duplicate hands!")
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_problem(input: String, with_wildcards: bool) -> u64 {
    let mut hands: Vec<Hand> = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<&str>>();
            Hand {
                cards: parts[0]
                    .chars()
                    .map(|c| match c {
                        'A' => 14,
                        'K' => 13,
                        'Q' => 12,
                        'J' => {
                            if with_wildcards {
                                WILDCARD
                            } else {
                                11
                            }
                        }
                        'T' => 10,
                        _ => c.to_digit(10).unwrap(),
                    })
                    .collect(),
                bid: parts[1].parse().unwrap(),
            }
        })
        .collect();

    hands.sort();

    let answer = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u64 * hand.bid as u64)
        .sum();

    // Check for wrong answers
    assert_ne!(answer, 249272171);

    answer
}

fn main() {
    let content = fs::read_to_string("../input").expect("Should have been able to read the file");

    let solution = solve_problem(content.clone(), false);
    println!("Solution part 1: {}", solution);

    let solution = solve_problem(content, true);
    println!("Solution part 2: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_1_solved() {
        let content = fs::read_to_string("../input-example-1")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem(content, false), 6440);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem(content, true), 5905);
    }

    #[test]
    fn problem_2_solved_custom_test() {
        let content = "
2345A 2
2345J 5
J345A 3
32T3K 7
T55J5 17
KK677 11
KTJJT 23
QQQJA 19
JJJJJ 29
JAAAA 37
AAAAJ 43
AAAAA 53
2AAAA 13
2JJJJ 41
JJJJ2 31
"
        .to_string();
        assert_eq!(solve_problem(content, true), 3667);
    }

    #[test]
    fn problem_2_solved_custom_test_2() {
        let content = "
AAAAA 2
22222 3
AAAAK 5
22223 7
AAAKK 11
22233 13
AAAKQ 17
22234 19
AAKKQ 23
22334 29
AAKQJ 31
22345 37
AKQJT 41
23456 43
"
        .to_string();
        assert_eq!(solve_problem(content, true), 1369);
    }

    #[test]
    fn problem_2_regression_test() {
        let content = "
JAAKK 1
JJJAK 2
"
        .to_string();
        assert_eq!(solve_problem(content, true), 5);
    }
}
