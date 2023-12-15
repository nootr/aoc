use std::fs;

#[derive(Debug, Default, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u32,
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

fn solve_problem_1(input: String) -> u64 {
    input.trim().split(',').map(|x| hash(x) as u64).sum()
}

fn solve_problem_2(input: String) -> u64 {
    let hashmap: Vec<Vec<Lens>> = input
        .trim()
        .split(',')
        .fold(vec![vec![]; 256], |mut acc, x| {
            if let Some(label) = x.strip_suffix('-') {
                // Remove lens from box
                let index = hash(label) as usize;
                acc[index].retain(|l| l.label != label);
            } else {
                // Add or replace lens in box
                assert!(x.contains('='));
                let parts: Vec<&str> = x.split('=').collect();
                let lens = Lens {
                    label: parts[0],
                    focal_length: parts[1].parse::<u32>().unwrap(),
                };
                let index = hash(lens.label) as usize;

                // Replace lens when it's already in the box
                let mut already_in_box = false;
                acc[index] = acc[index]
                    .iter()
                    .map(|l| {
                        if l.label == lens.label {
                            already_in_box = true;
                            lens.clone()
                        } else {
                            l.clone()
                        }
                    })
                    .collect();

                // Add lens when it's not in the box
                if !already_in_box {
                    acc[index].push(lens);
                }
            }
            acc
        });

    hashmap
        .iter()
        .enumerate()
        .flat_map(|(box_index, lenses)| {
            lenses.iter().enumerate().map(move |(lens_index, lens)| {
                (box_index as u64 + 1) * (lens_index as u64 + 1) * lens.focal_length as u64
            })
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
        let content = fs::read_to_string("../input-example-1")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_1(content), 1320);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 145);
    }
}
