fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(Game::from)
        .filter(|g| g.is_possible())
        .map(|g| g.id)
        .sum()
}

fn part_two(input: &str) -> i32 {
    input.lines().map(Game::from).map(|g| g.power()).sum()
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>,
}

impl Game {
    pub fn is_possible(&self) -> bool {
        for set in self.sets.iter() {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                return false;
            }
        }

        true
    }

    pub fn power(&self) -> i32 {
        let sets = &self.sets;

        let red = sets.iter().map(|s| s.red).max().unwrap();
        let green = sets.iter().map(|s| s.green).max().unwrap();
        let blue = sets.iter().map(|s| s.blue).max().unwrap();

        red * green * blue
    }
}

#[derive(Debug)]
struct Set {
    red: i32,
    green: i32,
    blue: i32,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut split = value.split(':');

        let id: i32 = split
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let sets: Vec<Set> = split.next().unwrap().split(';').map(Set::from).collect();

        Self { id, sets }
    }
}

impl From<&str> for Set {
    fn from(value: &str) -> Self {
        let cubes = value.split(',');

        let mut set = Self {
            red: 0,
            green: 0,
            blue: 0,
        };

        for cube in cubes {
            let mut split = cube.split_ascii_whitespace();
            let value: i32 = split.next().unwrap().parse().unwrap();
            let color = split.next().unwrap();

            match color {
                "red" => set.red = value,
                "green" => set.green = value,
                "blue" => set.blue = value,
                _ => panic!("unexpected color"),
            }
        }

        set
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../assets/two.txt");

    #[test]
    fn test_one() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part_one(input), 8)
    }

    #[test]
    fn solution_one() {
        let solution = part_one(INPUT);
        dbg!(solution);
    }

    #[test]
    fn test_two() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part_two(input), 2286)
    }

    #[test]
    fn solution_two() {
        let solution = part_two(INPUT);
        dbg!(solution);
    }
}
