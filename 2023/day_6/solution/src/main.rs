use std::fs;

struct Game {
    distance: u64, // mm
}

impl Game {
    fn won(&self, distance_to_beat: u64) -> bool {
        self.distance > distance_to_beat
    }
}

#[derive(Debug)]
struct Race {
    time: u64,            // ms
    record_distance: u64, // mm
}

impl Race {
    fn get_games(&self) -> Vec<Game> {
        let mut games = Vec::new();
        for i in 0..self.time {
            let speed = i;
            let time_left = self.time - i;

            games.push(Game {
                distance: time_left * speed,
            })
        }
        games
    }

    fn wins_count(&self) -> u64 {
        self.get_games()
            .iter()
            .filter(|g| g.won(self.record_distance))
            .count() as u64
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
