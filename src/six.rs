fn part_one(input: &str) -> u64 {
    let races = parse_input_one(input);

    races
        .iter()
        .map(|r| r.wins_count())
        .reduce(|acc, e| acc * e)
        .unwrap()
}

fn part_two(input: &str) -> u64 {
    let race = parse_input_two(input);
    let race = dbg!(race);

    race.wins_count()
}

fn parse_input_one(input: &str) -> Vec<Race> {
    let mut lines = input.lines();

    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut races = vec![];
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];

        races.push(Race { time, distance })
    }

    races
}

fn parse_input_two(input: &str) -> Race {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();

    Race { time, distance }
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn wins_count(&self) -> u64 {
        (0..self.time)
            .map(|hold| self.race_distance(hold))
            .filter(|d| *d > self.distance)
            .count() as u64
    }

    fn race_distance(&self, hold_duration: u64) -> u64 {
        let speed = hold_duration;
        let remaining_time = self.time - hold_duration;
        speed * remaining_time
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../assets/six.txt");
    const SAMPLE_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_one() {
        let res = part_one(SAMPLE_INPUT);
        assert_eq!(res, 288);
    }

    #[test]
    fn solution_one() {
        let res = part_one(INPUT);
        dbg!(res);
    }

    #[test]
    fn test_two() {
        let res = part_two(SAMPLE_INPUT);
        assert_eq!(res, 71503);
    }

    #[test]
    fn solution_two() {
        let res = part_two(INPUT);
        dbg!(res);
    }
}
