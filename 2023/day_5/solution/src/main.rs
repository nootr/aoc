use std::fs;

#[derive(Debug, Clone)]
struct SeedRange {
    start: u64,
    length: u64,
}

#[derive(Debug)]
struct Map {
    destination_start: u64,
    source_start: u64,
    range_length: u64,
}

impl Map {
    fn new(inputs: Vec<u64>) -> Map {
        Map {
            destination_start: inputs[0],
            source_start: inputs[1],
            range_length: inputs[2],
        }
    }

    fn convert(&self, input: u64) -> Option<u64> {
        if input >= self.source_start && input < self.source_start + self.range_length {
            Some(input - self.source_start + self.destination_start)
        } else {
            None
        }
    }

    fn retrace(&self, output: u64) -> Option<u64> {
        if output >= self.destination_start && output < self.destination_start + self.range_length {
            Some(output - self.destination_start + self.source_start)
        } else {
            None
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
    seeds: Vec<SeedRange>,
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
                    almanak.seeds = seeds_str
                        .split(' ')
                        .map(|x| x.parse::<u64>().unwrap())
                        .map(|seed| SeedRange {
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
                        almanak.seeds.push(SeedRange { start, length });
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

    fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = self
            .seed_to_soil
            .iter()
            .find_map(|map| map.convert(seed))
            .unwrap_or(seed);
        let fertilizer = self
            .soil_to_fertilizer
            .iter()
            .find_map(|map| map.convert(soil))
            .unwrap_or(soil);
        let water = self
            .fertilizer_to_water
            .iter()
            .find_map(|map| map.convert(fertilizer))
            .unwrap_or(fertilizer);
        let light = self
            .water_to_light
            .iter()
            .find_map(|map| map.convert(water))
            .unwrap_or(water);
        let temperature = self
            .light_to_temperature
            .iter()
            .find_map(|map| map.convert(light))
            .unwrap_or(light);
        let humidity = self
            .temperature_to_humidity
            .iter()
            .find_map(|map| map.convert(temperature))
            .unwrap_or(temperature);
        let location = self
            .humidity_to_location
            .iter()
            .find_map(|map| map.convert(humidity))
            .unwrap_or(humidity);
        location
    }

    fn location_to_seed(&self, location: u64) -> u64 {
        // TODO: unwrap_or instead of ok_or, u64 instead of result
        let humidity = self
            .humidity_to_location
            .iter()
            .find_map(|map| map.retrace(location))
            .unwrap_or(location);
        let temperature = self
            .temperature_to_humidity
            .iter()
            .find_map(|map| map.retrace(humidity))
            .unwrap_or(humidity);
        let light = self
            .light_to_temperature
            .iter()
            .find_map(|map| map.retrace(temperature))
            .unwrap_or(temperature);
        let water = self
            .water_to_light
            .iter()
            .find_map(|map| map.retrace(light))
            .unwrap_or(light);
        let fertilizer = self
            .fertilizer_to_water
            .iter()
            .find_map(|map| map.retrace(water))
            .unwrap_or(water);
        let soil = self
            .soil_to_fertilizer
            .iter()
            .find_map(|map| map.retrace(fertilizer))
            .unwrap_or(fertilizer);
        self.seed_to_soil
            .iter()
            .find_map(|map| map.retrace(soil))
            .unwrap_or(soil)
    }

    fn lowest_location_seed(&self) -> u64 {
        let location_max = self
            .humidity_to_location
            .iter()
            .flat_map(|map| (map.destination_start..map.destination_start + map.range_length))
            .max()
            .unwrap_or_default();

        for location in 0..location_max {
            // Map each location to a humidity, to a temperature, etc., to a seed
            let seed = self.location_to_seed(location);

            // TODO: check if seed in seed range
            let seed_range = self
                .seeds
                .iter()
                .find(|range| seed >= range.start && seed < range.start + range.length);

            // If a seed is found, return it, else continue with the next location
            if seed_range.is_some() {
                return location;
            }
        }
        panic!("No seed found")
    }
}

fn solve_problem_1(input: String) -> u64 {
    let almanak = Almanak::new(input, false);
    almanak
        .seeds
        .iter()
        .flat_map(|range| (range.start..range.start + range.length))
        .map(|seed| almanak.seed_to_location(seed))
        .min()
        .unwrap()
}

fn solve_problem_2(input: String) -> u64 {
    let almanak = Almanak::new(input, true);
    almanak.lowest_location_seed()
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
