use std::fs;

fn main() {
    let content = fs::read_to_string("../input")
        .expect("Should have been able to read the file");

    let sum = content
        .clone()
        .split("\n")
        .map(|x| x.chars().filter(|x| x.is_digit(10)).collect::<String>())
        .filter(|x| !x.is_empty())
        .map(|x| format!("{}{}", &x[..1], &x[x.len() - 1..]))
        .map(|x| x.parse::<i64>().unwrap())
        .sum::<i64>();
    println!("Part 1: {}", sum);

    let sum = content
        .split("\n")
        .map(|x| {
            let mut first_digit = None;
            let mut last_digit = None;
            let mut i = 0;
            while i < x.len() {
                let y = &x[i..];
                if y.starts_with("1") {
                    first_digit = Some("1".to_string());
                    break;
                } else if y.starts_with("2") {
                    first_digit = Some("2".to_string());
                    break;
                } else if y.starts_with("3") {
                    first_digit = Some("3".to_string());
                    break;
                } else if y.starts_with("4") {
                    first_digit = Some("4".to_string());
                    break;
                } else if y.starts_with("5") {
                    first_digit = Some("5".to_string());
                    break;
                } else if y.starts_with("6") {
                    first_digit = Some("6".to_string());
                    break;
                } else if y.starts_with("7") {
                    first_digit = Some("7".to_string());
                    break;
                } else if y.starts_with("8") {
                    first_digit = Some("8".to_string());
                    break;
                } else if y.starts_with("9") {
                    first_digit = Some("9".to_string());
                    break;
                } else if y.starts_with("0") {
                    first_digit = Some("0".to_string());
                    break;
                } else if y.starts_with("one") {
                    first_digit = Some("1".to_string());
                    break;
                } else if y.starts_with("two") {
                    first_digit = Some("2".to_string());
                    break;
                } else if y.starts_with("three") {
                    first_digit = Some("3".to_string());
                    break;
                } else if y.starts_with("four") {
                    first_digit = Some("4".to_string());
                    break;
                } else if y.starts_with("five") {
                    first_digit = Some("5".to_string());
                    break;
                } else if y.starts_with("six") {
                    first_digit = Some("6".to_string());
                    break;
                } else if y.starts_with("seven") {
                    first_digit = Some("7".to_string());
                    break;
                } else if y.starts_with("eight") {
                    first_digit = Some("8".to_string());
                    break;
                } else if y.starts_with("nine") {
                    first_digit = Some("9".to_string());
                    break;
                } else {
                    i += 1;
                }
            }
            let mut i = x.len();
            loop {
                let y = &x[i..];
                if y.starts_with("1") {
                    last_digit = Some("1".to_string());
                    break;
                } else if y.starts_with("2") {
                    last_digit = Some("2".to_string());
                    break;
                } else if y.starts_with("3") {
                    last_digit = Some("3".to_string());
                    break;
                } else if y.starts_with("4") {
                    last_digit = Some("4".to_string());
                    break;
                } else if y.starts_with("5") {
                    last_digit = Some("5".to_string());
                    break;
                } else if y.starts_with("6") {
                    last_digit = Some("6".to_string());
                    break;
                } else if y.starts_with("7") {
                    last_digit = Some("7".to_string());
                    break;
                } else if y.starts_with("8") {
                    last_digit = Some("8".to_string());
                    break;
                } else if y.starts_with("9") {
                    last_digit = Some("9".to_string());
                    break;
                } else if y.starts_with("0") {
                    last_digit = Some("0".to_string());
                    break;
                } else if y.starts_with("one") {
                    last_digit = Some("1".to_string());
                    break;
                } else if y.starts_with("two") {
                    last_digit = Some("2".to_string());
                    break;
                } else if y.starts_with("three") {
                    last_digit = Some("3".to_string());
                    break;
                } else if y.starts_with("four") {
                    last_digit = Some("4".to_string());
                    break;
                } else if y.starts_with("five") {
                    last_digit = Some("5".to_string());
                    break;
                } else if y.starts_with("six") {
                    last_digit = Some("6".to_string());
                    break;
                } else if y.starts_with("seven") {
                    last_digit = Some("7".to_string());
                    break;
                } else if y.starts_with("eight") {
                    last_digit = Some("8".to_string());
                    break;
                } else if y.starts_with("nine") {
                    last_digit = Some("9".to_string());
                    break;
                } else {
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                }
            }
            match (first_digit, last_digit) {
                (Some(x), Some(y)) => {
                    format!("{}{}", x, y)
                },
                _ => {
                    "0".to_string()
                }
            }
        })
        .filter( |x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .sum::<i64>();
    println!("Part 2: {:?}", sum);

}
