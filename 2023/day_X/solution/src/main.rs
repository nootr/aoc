use std::fs;

fn solve_problem_1(input: String) -> u32 {
    0
}

fn solve_problem_2(input: String) -> u32 {
    0
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
TBD
",
        );
        assert_eq!(solve_problem_1(input), 1234);
    }

    #[test]
    fn problem_2_solved() {
        return;  // Remove me
        let input = String::from(
            "
TBD
",
        );
        assert_eq!(solve_problem_2(input), 5678);
    }
}
