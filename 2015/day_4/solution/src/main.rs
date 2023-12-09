use std::fs;

fn solve_problem_1(input: String) -> u64 {
    let prefix = input.trim();

    let mut i = 0;
    loop {
        let hash = md5::compute(format!("{}{:05}", prefix, i));
        let hash = format!("{:x}", hash);
        if hash.starts_with("00000") {
            return i;
        }
        i += 1;
    }
}

fn solve_problem_2(input: String) -> u64 {
    let prefix = input.trim();

    let mut i = 0;
    loop {
        let hash = md5::compute(format!("{}{:05}", prefix, i));
        let hash = format!("{:x}", hash);
        if hash.starts_with("000000") {
            return i;
        }
        i += 1;
    }
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
        assert_eq!(solve_problem_1(content), 609043);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 5678);
    }
}
