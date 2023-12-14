use std::fs;

fn solve_problem_1(input: String) -> u64 {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.parse::<u64>().unwrap()).sum())
        .max()
        .unwrap()
}

fn solve_problem_2(input: String) -> u64 {
    let mut sums = input
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.parse::<u64>().unwrap()).sum())
        .collect::<Vec<u64>>();
    sums.sort();
    sums[sums.len() - 3..].iter().sum()
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
        assert_eq!(solve_problem_1(content), 24000);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 45000);
    }
}
