use std::cmp::{max, min};
use std::fs;

#[derive(Debug, Clone)]
struct Range {
    start: u64,
    length: u64,
}

impl Range {
    fn end(&self) -> u64 {
        self.start + self.length
    }
}

#[derive(Debug)]
struct RangeMapResult {
    mapped_range: Option<Range>,
    remaining_ranges: Vec<Range>,
}

#[derive(Debug)]
struct Map {
    source: Range,
    destination: Range,
}

impl Map {
    fn new(inputs: Vec<u64>) -> Map {
        Map {
            source: Range {
                start: inputs[1],
                length: inputs[2],
            },
            destination: Range {
                start: inputs[0],
                length: inputs[2],
            },
        }
    }

    /// Returns the destination range if the given range overlaps with the source range.
    fn map_range(&self, range: &Range) -> RangeMapResult {
        let mut remaining_ranges = vec![];

        if range.start < self.source.start {
            remaining_ranges.push(Range {
                start: range.start,
                length: self.source.start - range.start,
            });
        }
        if range.end() > self.source.end() {
            remaining_ranges.push(Range {
                start: self.source.end() + 1,
                length: range.end() - self.source.end() - 1,
            });
        }
        let mapped_range_length: i64 =
            min(range.end(), self.source.end()) as i64 - max(range.start, self.source.start) as i64;
        let mapped_range = if mapped_range_length > 0 {
            Some(Range {
                start: max(range.start, self.source.start) + self.destination.start
                    - self.source.start,
                length: mapped_range_length as u64,
            })
        } else {
            None
        };
        RangeMapResult {
            mapped_range,
            remaining_ranges,
        }
    }
}

#[derive(Debug)]
enum ParseState {
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug, Default)]
struct Almanak {
    seed_ranges: Vec<Range>,
    seed_to_soil: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temperature: Vec<Map>,
    temperature_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

impl Almanak {
    fn new(input: String, seed_range: bool) -> Self {
        let mut state = ParseState::Seeds;
        let mut almanak = Almanak::default();
        for line in input.lines().filter(|x| !x.is_empty()) {
            // Set next state
            match line {
                "seed-to-soil map:" => {
                    state = ParseState::SeedToSoil;
                    continue;
                }
                "soil-to-fertilizer map:" => {
                    state = ParseState::SoilToFertilizer;
                    continue;
                }
                "fertilizer-to-water map:" => {
                    state = ParseState::FertilizerToWater;
                    continue;
                }
                "water-to-light map:" => {
                    state = ParseState::WaterToLight;
                    continue;
                }
                "light-to-temperature map:" => {
                    state = ParseState::LightToTemperature;
                    continue;
                }
                "temperature-to-humidity map:" => {
                    state = ParseState::TemperatureToHumidity;
                    continue;
                }
                "humidity-to-location map:" => {
                    state = ParseState::HumidityToLocation;
                    continue;
                }
                _ => {}
            }

            // Parse line
            match state {
                ParseState::Seeds if !seed_range => {
                    let seeds_str = line.split(": ").collect::<Vec<&str>>()[1];
                    almanak.seed_ranges = seeds_str
                        .split(' ')
                        .map(|x| x.parse::<u64>().unwrap())
                        .map(|seed| Range {
                            start: seed,
                            length: 1,
                        })
                        .collect()
                }
                ParseState::Seeds if seed_range => {
                    let seeds_str = line.split(": ").collect::<Vec<&str>>()[1];
                    let seeds_and_ranges: Vec<u64> = seeds_str
                        .split(' ')
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect();
                    for chunk in seeds_and_ranges.chunks(2) {
                        let start = chunk[0];
                        let length = chunk[1];
                        almanak.seed_ranges.push(Range { start, length });
                    }
                }
                _ => {
                    let inputs: Vec<u64> = line.split(' ').map(|x| x.parse().unwrap()).collect();
                    let map = Map::new(inputs);
                    match state {
                        ParseState::SeedToSoil => {
                            almanak.seed_to_soil.push(map);
                        }
                        ParseState::SoilToFertilizer => {
                            almanak.soil_to_fertilizer.push(map);
                        }
                        ParseState::FertilizerToWater => {
                            almanak.fertilizer_to_water.push(map);
                        }
                        ParseState::WaterToLight => {
                            almanak.water_to_light.push(map);
                        }
                        ParseState::LightToTemperature => {
                            almanak.light_to_temperature.push(map);
                        }
                        ParseState::TemperatureToHumidity => {
                            almanak.temperature_to_humidity.push(map);
                        }
                        ParseState::HumidityToLocation => {
                            almanak.humidity_to_location.push(map);
                        }
                        _ => panic!("Unexpected state"),
                    }
                }
            }
        }
        almanak
    }

    /// Returns the lowest location number for the given seed ranges.
    fn lowest_location_for_seeds(&self) -> u64 {
        let mut current_ranges = self.seed_ranges.clone();

        let map_groups = vec![
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ];

        for map_group in map_groups {
            let mut next_ranges: Vec<Range> = vec![];
            while !current_ranges.is_empty() {
                let range = current_ranges.remove(0);
                let mut matched = false;
                for map in map_group.iter() {
                    let result = map.map_range(&range);
                    if let Some(mapped_range) = result.mapped_range {
                        next_ranges.push(mapped_range);
                        current_ranges.extend(result.remaining_ranges);
                        matched = true;
                        break;
                    }
                }
                if !matched {
                    next_ranges.push(range);
                }
            }
            current_ranges.extend(next_ranges);
        }

        current_ranges
            .iter()
            .map(|range| range.start)
            .min()
            .unwrap()
    }
}

fn solve_problem_1(input: String) -> u64 {
    let almanak = Almanak::new(input, false);
    almanak.lowest_location_for_seeds()
}

fn solve_problem_2(input: String) -> u64 {
    let almanak = Almanak::new(input, true);
    almanak.lowest_location_for_seeds()
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
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
",
        );
        assert_eq!(solve_problem_1(input), 35);
    }

    #[test]
    fn problem_2_solved() {
        let input = String::from(
            "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
",
        );
        assert_eq!(solve_problem_2(input), 46);
    }
}
