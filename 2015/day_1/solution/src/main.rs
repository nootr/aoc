use std::fs;

fn solve_problem_1(input: String) -> i32 {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum()
}

fn solve_problem_2(input: String) -> i32 {
    let mut floor = 0;
    for (i, x) in input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .enumerate()
    {
        floor += x;
        if floor == -1 {
            return (i + 1) as i32;
        }
    }
    panic!("Couldn't find answer")
}

fn main() {
    let content = fs::read_to_string("../input").expect("Should have been able to read the file");

    let solution = solve_problem_1(content.clone());
    println!("Solution part 1: {}", solution);

    let solution = solve_problem_2(content);
    println!("Solution part 2: {}", solution);
}
