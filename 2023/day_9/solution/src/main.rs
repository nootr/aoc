use std::fs;

enum PredictionType {
    Forward,
    Backward,
}

#[derive(Debug)]
struct Sequence {
    numbers: Vec<i64>,
}

impl Sequence {
    fn new(data: &str) -> Self {
        Self {
            numbers: data.split(' ').map(|x| x.parse().unwrap()).collect(),
        }
    }

    fn prediction_recursive(numbers: Vec<i64>, prediction_type: PredictionType) -> i64 {
        let mut new_numbers = vec![];
        let mut all_zeroes = true;
        let mut last_number = None;

        for number in numbers.iter() {
            if let Some(last_number) = last_number {
                let new_number = number - last_number;
                if new_number != 0 {
                    all_zeroes = false;
                }
                new_numbers.push(new_number);
            }
            last_number = Some(*number);
        }
        match prediction_type {
            PredictionType::Forward if all_zeroes => *numbers.last().unwrap(),
            PredictionType::Backward if all_zeroes => *numbers.first().unwrap(),
            PredictionType::Forward => {
                numbers.last().unwrap() + Self::prediction_recursive(new_numbers, prediction_type)
            }
            PredictionType::Backward => {
                numbers.first().unwrap() - Self::prediction_recursive(new_numbers, prediction_type)
            }
        }
    }

    fn predict_next(&self) -> i64 {
        Self::prediction_recursive(self.numbers.clone(), PredictionType::Forward)
    }

    fn predict_last(&self) -> i64 {
        Self::prediction_recursive(self.numbers.clone(), PredictionType::Backward)
    }
}

fn solve_problem_1(input: String) -> i64 {
    let sequences: Vec<Sequence> = input.lines().map(Sequence::new).collect();
    sequences.iter().map(|x| x.predict_next()).sum()
}

fn solve_problem_2(input: String) -> i64 {
    let sequences: Vec<Sequence> = input.lines().map(Sequence::new).collect();
    sequences.iter().map(|x| x.predict_last()).sum()
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
        assert_eq!(solve_problem_1(content), 114);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 2);
    }
}
