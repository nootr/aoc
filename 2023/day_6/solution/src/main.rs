use std::cmp::{max, min};
use std::fs;

#[derive(Debug)]
struct Race {
    time: u64,            // ms
    record_distance: u64, // mm
}

impl Race {
    /// Returns the number of possible games that win.
    ///
    /// A boat reaches a certain distance in the given time:
    ///
    /// distance(holding_time) = time_left * speed
    ///                        = (time - holding_time) * speed
    ///                        = (time - holding_time) * holding_time
    ///                        = -holding_time^2 + (time * holding_time)
    ///
    /// or:
    ///
    /// f(x) = -x^2 + Tx
    ///      = W
    ///
    /// Assuming there are winners (i.e. distance > record_distance), there should be two
    /// intersections on `f(x) = W`, where `W = record_distance`.
    ///
    /// We have to calculate the two intersections and count the number of whole numbers
    /// in the x-axis between them.
    ///
    /// x^2 - Tx + W = 0            x: holding_time, T: race.time, W: race.record_distance
    /// ax^2 + bx + c = 0 => a = 1, b = -T, c = W
    ///
    /// The quadratic formula gives us two solutions:
    ///
    /// x1, x2 = (-b +- sqrt(b^2 - 4ac)) / 2a
    ///        = (T +- sqrt(T^2 - 4W)) / 2
    ///
    fn wins_count(&self) -> u64 {
        let winning_holding_time_min = (self.time as f64
            - ((self.time * self.time - 4 * self.record_distance) as f64).sqrt())
            / 2.0;
        let winning_holding_time_max = (self.time as f64
            + ((self.time * self.time - 4 * self.record_distance) as f64).sqrt())
            / 2.0;

        let mut _min = max(0, winning_holding_time_min.ceil() as u64);
        let mut _max = min(self.time, winning_holding_time_max.floor() as u64);

        if _min as f64 == winning_holding_time_min {
            _min += 1;
        }

        if _max as f64 == winning_holding_time_max {
            _max -= 1;
        }

        _max - _min + 1
    }
}

fn solve_problem_1(input: String) -> u64 {
    let data: Vec<Vec<&str>> = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| line.split(':').collect::<Vec<&str>>()[1])
        .map(|l| l.split(' ').filter(|x| !x.is_empty()).collect())
        .collect();

    let times = &data[0];
    let distances = &data[1];
    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| Race {
            time: t.parse::<u64>().unwrap(),
            record_distance: d.parse::<u64>().unwrap(),
        })
        .collect::<Vec<Race>>();

    races.iter().map(Race::wins_count).product()
}

fn solve_problem_2(input: String) -> u64 {
    let data: Vec<String> = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| line.split(':').collect::<Vec<&str>>()[1])
        .map(|s| s.chars().filter(|c| !c.is_whitespace()).collect::<String>())
        .collect();

    let time = &data[0];
    let distance = &data[1];
    Race {
        time: time.parse::<u64>().unwrap(),
        record_distance: distance.parse::<u64>().unwrap(),
    }
    .wins_count()
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
        assert_eq!(solve_problem_1(content), 288);
    }

    #[test]
    fn problem_2_solved() {
        let content = fs::read_to_string("../input-example-2")
            .expect("Should have been able to read the file");
        assert_eq!(solve_problem_2(content), 71503);
    }
}
