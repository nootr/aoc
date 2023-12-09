use std::collections::HashSet;
use std::fs;

fn string_is_nice(string: &str) -> bool {
    let mut vowel_count = 0;
    let mut contains_double = false;
    let mut last_char: Option<char> = None;

    for c in string.chars() {
        if "aeiou".contains(c) {
            vowel_count += 1;
        }
        if let Some(last_char) = last_char {
            if last_char == c {
                contains_double = true;
            }
            match format!("{}{}", last_char, c).as_str() {
                "ab" | "cd" | "pq" | "xy" => return false,
                _ => (),
            }
        }
        last_char = Some(c);
    }
    vowel_count >= 3 && contains_double
}

fn string_is_really_nice(string: &str) -> bool {
    let mut contains_pair = false;
    let mut contains_double = false;

    let mut pairs = HashSet::new();

    let mut last_char: Option<char> = None;
    let mut last_last_char: Option<char> = None;
    let mut last_pair: Option<String> = None;

    for c in string.chars() {
        if let Some(last_last_char) = last_last_char {
            if last_last_char == c {
                contains_double = true;
            }
        }
        if let Some(last_char) = last_char {
            let pair = format!("{}{}", last_char, c);
            let mut overlap = false;
            if let Some(ref last_pair) = last_pair {
                if last_pair.clone() == pair {
                    overlap = true;
                }
            }
            if pairs.contains(&pair) && !overlap {
                contains_pair = true;
            } else {
                pairs.insert(pair.clone());
            }
            if overlap {
                last_pair = None;
            } else {
                last_pair = Some(pair);
            }
        }
        last_last_char = last_char;
        last_char = Some(c);
    }
    contains_double && contains_pair
}

fn solve_problem_1(input: String) -> usize {
    input.lines().filter(|s| string_is_nice(s)).count()
}

fn solve_problem_2(input: String) -> usize {
    input.lines().filter(|s| string_is_really_nice(s)).count()
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
        assert_eq!(solve_problem_1(content), 2);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 2);
    }

    #[test]
    fn solves_aaa() {
        let content = "aaa".to_string();
        assert_eq!(solve_problem_2(content), 0);
    }

    #[test]
    fn solves_aaaa() {
        let content = "aaaa".to_string();
        assert_eq!(solve_problem_2(content), 1);
    }
}
