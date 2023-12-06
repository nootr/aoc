use std::cmp::min;
use std::fs;

fn solve_problem_1(input: String) -> u32 {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            let sizes = line
                .split('x')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>();
            let l = sizes[0];
            let w = sizes[1];
            let h = sizes[2];
            let a = l * w;
            let b = l * h;
            let c = w * h;
            2 * a + 2 * b + 2 * c + min(min(a, b), c)
        })
        .sum()
}

fn solve_problem_2(input: String) -> u32 {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            let sizes = line
                .split('x')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>();
            let l = sizes[0];
            let w = sizes[1];
            let h = sizes[2];
            let a = 2 * (l + w);
            let b = 2 * (l + h);
            let c = 2 * (w + h);
            min(min(a, b), c) + l * w * h
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
2x3x4
1x1x10
",
        );
        assert_eq!(solve_problem_1(input), 58 + 43);
    }

    #[test]
    fn problem_2_solved() {
        let input = String::from(
            "
2x3x4
1x1x10
",
        );
        assert_eq!(solve_problem_2(input), 34 + 14);
    }
}
