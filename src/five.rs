use std::ops::Range;

fn part_one(input: &str) -> usize {
    let (seeds, maps) = parse_input(input);

    seeds
        .iter()
        .map(|seed| {
            let mut source = *seed;
            for map in &maps {
                source = map.to_destination(source);
            }

            source
        })
        .min()
        .unwrap()
}

fn part_two(input: &str) -> usize {
    let (seed_ranges, maps) = parse_input(input);

    let ranges: Vec<Range<usize>> = seed_ranges
        .chunks(2)
        .map(|item| item[0]..item[0] + item[1])
        .collect();

    for lowest in 0..usize::MAX {
        let mut destination = lowest;
        for map in maps.iter().clone().rev() {
            destination = map.to_source(destination);
        }

        for range in &ranges {
            if range.contains(&destination) {
                return destination;
            }
        }
    }

    unreachable!()
}

fn process_seed_range(seeds: &Range<usize>, maps: &[Map]) -> usize {
    seeds
        .clone()
        .map(|seed| {
            let mut source = seed;
            for map in maps {
                source = map.to_destination(source);
            }

            source
        })
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Map>) {
    let mut lines = input.split("\n\n");

    // Seeds
    let seeds = lines.next().unwrap().split(':').last().unwrap();
    let seeds = seeds
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Maps
    let maps = lines.map(Map::from).collect();

    (seeds, maps)
}

#[derive(Debug)]
struct Map(Vec<Instruction>);

impl Map {
    pub fn to_destination(&self, value: usize) -> usize {
        for instruction in &self.0 {
            if let Some(destination) = instruction.to_destination(value) {
                return destination;
            }
        }

        value
    }

    pub fn to_source(&self, destination: usize) -> usize {
        for instruction in &self.0 {
            if let Some(source) = instruction.to_source(destination) {
                return source;
            }
        }

        destination
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let instructions = value.lines().skip(1).map(Instruction::from).collect();
        Self(instructions)
    }
}

#[derive(Debug)]
struct Instruction {
    source: usize,
    destination: usize,
    step: usize,
}

impl Instruction {
    pub fn to_destination(&self, value: usize) -> Option<usize> {
        // Requested value not in source range
        let range = self.source..self.source + self.step;
        if !range.contains(&value) {
            return None;
        }

        let value = value - self.source;

        Some(self.destination + value)
    }

    pub fn to_source(&self, destination: usize) -> Option<usize> {
        let range = self.destination..self.destination + self.step;
        if !range.contains(&destination) {
            return None;
        }

        let dif = destination - self.destination;

        Some(self.source + dif)
    }
}
impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut split = value.split_ascii_whitespace();

        let destination = split.next().unwrap().parse().unwrap();
        let source = split.next().unwrap().parse().unwrap();
        let step = split.next().unwrap().parse().unwrap();

        Self {
            source,
            destination,
            step,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../assets/five.txt");

    const SAMPLE_INPUT: &str = "seeds: 79 14 55 13

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
56 93 4";

    #[test]
    fn test_one() {
        assert_eq!(part_one(SAMPLE_INPUT), 35);
    }

    #[test]
    fn solution_one() {
        let res = part_one(INPUT);
        dbg!(res);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(SAMPLE_INPUT), 46);
    }

    #[test]
    fn solution_two() {
        let res = part_two(INPUT);
        dbg!(res);
    }
}
