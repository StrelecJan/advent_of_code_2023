use std::collections::HashMap;

fn part_one(input: &str) -> i32 {
    input.lines().map(Card::from).map(|c| c.score()).sum()
}

fn part_two(input: &str) -> i32 {
    let cards = input.lines().map(Card::from);
    let cards_count = cards.clone().count();
    let mut instances = HashMap::new();

    // Fill instances
    cards.clone().for_each(|c| {
        instances.insert(c.id, 1);
    });

    for card in cards {
        let count = *instances.get(&card.id).unwrap();
        let matching_count = card.matching_numbers();

        for idx in 1..=matching_count {
            if idx as usize >= cards_count {
                break;
            }
            let id = card.id + idx;
            let next_instance = instances.get_mut(&id).unwrap();
            *next_instance += count;
        }
    }

    instances.values().sum()
}

struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    scratched_numbers: Vec<i32>,
}

impl Card {
    pub fn score(&self) -> i32 {
        let mut score = 0;
        for winning_number in &self.winning_numbers {
            if self.scratched_numbers.contains(winning_number) {
                if score == 0 {
                    score = 1
                } else {
                    score *= 2
                }
            }
        }

        score
    }

    pub fn matching_numbers(&self) -> i32 {
        let mut matching = 0;
        for winning_number in &self.winning_numbers {
            if self.scratched_numbers.contains(winning_number) {
                if matching == 0 {
                    matching = 1
                } else {
                    matching += 1
                }
            }
        }

        matching
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let mut split = value.split(':');

        let id: i32 = split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let mut numbers = split.next().unwrap().split('|');

        let winning_numbers: Vec<i32> = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let scratched_numbers: Vec<i32> = numbers
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        Self {
            id,
            winning_numbers,
            scratched_numbers,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../assets/four.txt");

    #[test]
    fn test_one() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part_one(input), 13)
    }

    #[test]
    fn solution_one() {
        let res = part_one(INPUT);
        dbg!(res);
    }

    #[test]
    fn test_two() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part_two(input), 30)
    }

    #[test]
    fn solution_two() {
        let res = part_two(INPUT);
        dbg!(res);
    }
}
