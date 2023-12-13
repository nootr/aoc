use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, Clone)]
struct Signal {
    name: String,
}

impl Signal {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Constant(u16),
    Variable(Signal),
}

#[derive(Debug, PartialEq)]
enum Token {
    Value(Value),
    And,
    Or,
    Not,
    LShift,
    RShift,
    Arrow,
}

#[derive(Debug)]
enum Instruction {
    Assign(Value),
    And(Value, Value),
    Or(Value, Value),
    Not(Value),
    LShift(Value, Value),
    RShift(Value, Value),
}

fn execute(
    instructions: &HashMap<String, Instruction>,
    signal: &str,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    if let Some(x) = cache.get(signal) {
        return *x;
    }
    let value = match instructions.get(signal).unwrap() {
        Instruction::Assign(value) => match value {
            Value::Constant(x) => *x,
            Value::Variable(signal) => execute(instructions, &signal.name, cache),
        },
        Instruction::And(left, right) => {
            let left = match left {
                Value::Constant(x) => *x,
                Value::Variable(signal) => execute(instructions, &signal.name, cache),
            };
            let right = match right {
                Value::Constant(x) => *x,
                Value::Variable(signal) => execute(instructions, &signal.name, cache),
            };
            left & right
        }
        Instruction::Or(left, right) => {
            let left = match left {
                Value::Constant(x) => *x,
                Value::Variable(signal) => execute(instructions, &signal.name, cache),
            };
            let right = match right {
                Value::Constant(x) => *x,
                Value::Variable(signal) => execute(instructions, &signal.name, cache),
            };
            left | right
        }
        Instruction::LShift(left, right) => {
            let left = match left {
                Value::Constant(x) => *x,
                Value::Variable(signal) => execute(instructions, &signal.name, cache),
            };
            let right = match right {
                Value::Constant(x) => *x,
                Value::Variable(signal) => execute(instructions, &signal.name, cache),
            };
            left << right
        }
        Instruction::RShift(left, right) => {
            let left = match left {
                Value::Constant(x) => *x,
                Value::Variable(signal) => execute(instructions, &signal.name, cache),
            };
            let right = match right {
                Value::Constant(x) => *x,
                Value::Variable(signal) => execute(instructions, &signal.name, cache),
            };
            left >> right
        }
        Instruction::Not(value) => {
            let value = match value {
                Value::Constant(x) => *x,
                Value::Variable(signal) => execute(instructions, &signal.name, cache),
            };
            !value
        }
    };
    cache.insert(signal.to_string(), value);
    value
}

fn parse_instructions(input: String) -> HashMap<String, Instruction> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|token| match token {
                    "AND" => Token::And,
                    "OR" => Token::Or,
                    "NOT" => Token::Not,
                    "LSHIFT" => Token::LShift,
                    "RSHIFT" => Token::RShift,
                    "->" => Token::Arrow,
                    value if value.parse::<u16>().is_ok() => {
                        Token::Value(Value::Constant(value.parse().unwrap()))
                    }
                    value => Token::Value(Value::Variable(Signal::new(value))),
                })
                .collect::<Vec<Token>>()
        })
        .fold(HashMap::new(), |mut acc, tokens| {
            let signal = match tokens.last().unwrap() {
                Token::Value(Value::Variable(signal)) => signal.name.clone(),
                _ => panic!("Syntax error, expected variable at end of line."),
            };
            if tokens[0] == Token::Not {
                let value = match &tokens[1] {
                    Token::Value(x) => x.clone(),
                    _ => panic!("Syntax error, expected value after NOT."),
                };
                acc.insert(signal, Instruction::Not(value));
            } else if tokens[1] == Token::Arrow {
                let value = match &tokens[0] {
                    Token::Value(x) => x.clone(),
                    _ => panic!("Syntax error, expected value before ->."),
                };
                acc.insert(signal, Instruction::Assign(value));
            } else if tokens[1] == Token::And {
                let left = match &tokens[0] {
                    Token::Value(x) => x.clone(),
                    _ => panic!("Syntax error, expected value before AND."),
                };
                let right = match &tokens[2] {
                    Token::Value(x) => x.clone(),
                    _ => panic!("Syntax error, expected value after AND."),
                };
                acc.insert(signal, Instruction::And(left, right));
            } else if tokens[1] == Token::Or {
                let left = match &tokens[0] {
                    Token::Value(x) => x.clone(),
                    _ => panic!("Syntax error, expected value before OR."),
                };
                let right = match &tokens[2] {
                    Token::Value(x) => x.clone(),
                    _ => panic!("Syntax error, expected value after OR."),
                };
                acc.insert(signal, Instruction::Or(left, right));
            } else if tokens[1] == Token::LShift {
                let left = match &tokens[0] {
                    Token::Value(x) => x.clone(),
                    _ => panic!("Syntax error, expected value before LSHIFT."),
                };
                let right = match &tokens[2] {
                    Token::Value(x) => x.clone(),
                    _ => panic!("Syntax error, expected value after LSHIFT."),
                };
                acc.insert(signal, Instruction::LShift(left, right));
            } else if tokens[1] == Token::RShift {
                let left = match &tokens[0] {
                    Token::Value(x) => x.clone(),
                    _ => panic!("Syntax error, expected value before RSHIFT."),
                };
                let right = match &tokens[2] {
                    Token::Value(x) => x.clone(),
                    _ => panic!("Syntax error, expected value after RSHIFT."),
                };
                acc.insert(signal, Instruction::RShift(left, right));
            } else {
                unreachable!()
            }
            acc
        })
}

fn solve_problem_1(input: String, signal: &str) -> u16 {
    let mut cache = HashMap::new();
    let instructions = parse_instructions(input);
    execute(&instructions, signal, &mut cache)
}

fn solve_problem_2(input: String) -> u16 {
    let mut cache = HashMap::new();
    let instructions = parse_instructions(input);
    let value = execute(&instructions, "a", &mut cache);
    let mut cache = HashMap::new();
    cache.insert("b".to_string(), value);
    execute(&instructions, "a", &mut cache)
}

fn main() {
    let content = fs::read_to_string("../input").expect("Should have been able to read the file");

    let solution = solve_problem_1(content.clone(), "a");
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
        assert_eq!(solve_problem_1(content.clone(), "d"), 72);
        assert_eq!(solve_problem_1(content.clone(), "e"), 507);
        assert_eq!(solve_problem_1(content.clone(), "f"), 492);
        assert_eq!(solve_problem_1(content.clone(), "g"), 114);
        assert_eq!(solve_problem_1(content.clone(), "h"), 65412);
        assert_eq!(solve_problem_1(content.clone(), "i"), 65079);
        assert_eq!(solve_problem_1(content.clone(), "x"), 123);
        assert_eq!(solve_problem_1(content.clone(), "y"), 456);
    }
}
