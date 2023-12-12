use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum SpringState {
    Operational,
    Damaged,
}

#[derive(Debug)]
struct Spring {
    state: Vec<Option<SpringState>>,
    group_sizes: Vec<usize>,
    possibilities_cache: HashMap<String, usize>,
}

impl Spring {
    fn new(input: &str, copies: usize) -> Self {
        let parts: Vec<&str> = input.split(' ').collect();
        let mut state = parts[0]
            .chars()
            .map(|c| match c {
                '.' => Some(SpringState::Operational),
                '#' => Some(SpringState::Damaged),
                '?' => None,
                _ => panic!("Unexpected spring state: {}", c),
            })
            .collect::<Vec<Option<SpringState>>>();
        state.push(None);
        state = state.repeat(copies);
        state.pop();
        Self {
            state,
            group_sizes: parts[1]
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect::<Vec<usize>>()
                .repeat(copies),
            possibilities_cache: HashMap::new(),
        }
    }

    fn get_cache_key(
        state: &[Option<SpringState>],
        groups_to_check: Vec<usize>,
        current_group_size: usize,
    ) -> String {
        format!(
            "s{}g{}c{}",
            state
                .iter()
                .map(|s| match s {
                    None => "x",
                    Some(SpringState::Operational) => ".",
                    Some(SpringState::Damaged) => "#",
                })
                .collect::<String>(),
            groups_to_check
                .iter()
                .map(|x| format!("{}-", x))
                .collect::<String>(),
            current_group_size
        )
    }

    fn possibilities_count_recursive(
        &mut self,
        state: &[Option<SpringState>],
        groups_to_check: Vec<usize>,
        current_group_size: usize,
    ) -> usize {
        let cache_key = Self::get_cache_key(state, groups_to_check.clone(), current_group_size);

        if let Some(possibilities) = self.possibilities_cache.get(&cache_key) {
            return *possibilities;
        }

        if state.is_empty() {
            let result = if groups_to_check.is_empty()
                || (groups_to_check.len() == 1 && groups_to_check[0] == current_group_size)
            {
                1
            } else {
                0
            };
            self.possibilities_cache.insert(cache_key, result);
            return result;
        }

        let mut possibilities = 0;
        if !groups_to_check.is_empty()
            && current_group_size < groups_to_check[0]
            && state[0].unwrap_or(SpringState::Damaged) == SpringState::Damaged
        {
            possibilities += self.possibilities_count_recursive(
                &state[1..],
                groups_to_check.clone(),
                current_group_size + 1,
            );
        }
        if state[0].unwrap_or(SpringState::Operational) == SpringState::Operational {
            possibilities += if current_group_size == 0 {
                self.possibilities_count_recursive(&state[1..], groups_to_check.clone(), 0)
            } else if !groups_to_check.is_empty() && groups_to_check[0] == current_group_size {
                self.possibilities_count_recursive(&state[1..], groups_to_check[1..].to_vec(), 0)
            } else {
                0
            };
        }

        self.possibilities_cache.insert(cache_key, possibilities);
        possibilities
    }

    fn possibilities_count(&mut self) -> usize {
        self.possibilities_count_recursive(&self.state.clone(), self.group_sizes.clone(), 0)
    }
}

fn solve_problem_1(input: String) -> usize {
    let mut springs: Vec<Spring> = input.lines().map(|l| Spring::new(l, 1)).collect();
    springs.iter_mut().map(|s| s.possibilities_count()).sum()
}

fn solve_problem_2(input: String) -> usize {
    let mut springs: Vec<Spring> = input.lines().map(|l| Spring::new(l, 5)).collect();
    springs.iter_mut().map(|s| s.possibilities_count()).sum()
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
    fn custom_test_problem_1() {
        let content = ".#...?....???? 1,1,3".to_string();
        assert_eq!(solve_problem_1(content), 2);
    }

    #[test]
    fn custom_test_problem_1b() {
        let content = "?###???????? 3,2,1".to_string();
        assert_eq!(solve_problem_1(content), 10);
    }

    #[test]
    fn custom_test_problem_1c() {
        let content = "#????? 2,1".to_string();
        assert_eq!(solve_problem_1(content), 3);
    }

    #[test]
    fn custom_test_problem_2() {
        let content = "???.### 1,1,3".to_string();
        assert_eq!(solve_problem_2(content), 1);
    }

    #[test]
    fn problem_1_solved() {
        let content = fs::read_to_string("../input-example-1")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_1(content), 21);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 525152);
    }
}
