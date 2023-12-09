use std::{cmp::Ordering, collections::HashMap};

fn part_one(input: &str) -> u32 {
    let mut hands = parse_input(input);
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank as u32 + 1))
        .sum()
}

fn parse_input(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::from).collect()
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type != other.hand_type {
            return false;
        }

        let count = self
            .cards
            .iter()
            .zip(other.cards.iter())
            .filter(|(a, b)| a != b)
            .count();

        count > 0
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type).unwrap() {
            std::cmp::Ordering::Less => Some(Ordering::Less),
            std::cmp::Ordering::Greater => Some(Ordering::Greater),
            std::cmp::Ordering::Equal => {
                // Compare cards
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    match self_card.partial_cmp(other_card).unwrap() {
                        Ordering::Less => return Some(Ordering::Less),
                        Ordering::Greater => return Some(Ordering::Greater),
                        Ordering::Equal => continue,
                    }
                }

                panic!()
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Hand {}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut split = value.split_ascii_whitespace();

        let cards: Vec<Card> = split.next().unwrap().chars().map(Card::from).collect();
        let bid: u32 = split.next().unwrap().parse().unwrap();
        let hand_type = HandType::from(cards.as_ref());

        Self {
            cards,
            hand_type,
            bid,
        }
    }
}

/// The cards are ordered from the lowest to the highest.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => panic!("unexpected value"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandType {
    fn is_five_kind(counts: &[u8]) -> bool {
        counts[0] == 5
    }
    fn is_four_kind(counts: &[u8]) -> bool {
        counts[0] == 4
    }

    fn is_full_house(counts: &[u8]) -> bool {
        counts[0] == 3 && counts[1] == 2
    }

    fn is_three_kind(counts: &[u8]) -> bool {
        counts[0] == 3 && counts[1] == 1 && counts[2] == 1
    }

    fn is_two_pair(counts: &[u8]) -> bool {
        counts[0] == 2 && counts[1] == 2 && counts[2] == 1
    }

    fn is_one_pair(counts: &[u8]) -> bool {
        counts[0] == 2 && counts[1] == 1 && counts[2] == 1 && counts[3] == 1
    }

    fn count_cards(cards: &[Card]) -> Vec<u8> {
        // Get unique cards and their counts
        let mut map = HashMap::new();
        for card in cards {
            let entry = map.entry(card.clone()).or_insert(0_u8);
            *entry += 1;
        }

        // Map to vector and sort
        let mut counts: Vec<u8> = map.values().copied().collect();
        counts.sort();
        counts.reverse();

        counts
    }
}

impl From<&[Card]> for HandType {
    fn from(cards: &[Card]) -> Self {
        let cards = HandType::count_cards(cards);

        if HandType::is_five_kind(&cards) {
            return Self::FiveKind;
        }
        if HandType::is_four_kind(&cards) {
            return Self::FourKind;
        }
        if HandType::is_full_house(&cards) {
            return Self::FullHouse;
        }
        if HandType::is_three_kind(&cards) {
            return Self::ThreeKind;
        }
        if HandType::is_two_pair(&cards) {
            return Self::TwoPair;
        }
        if HandType::is_one_pair(&cards) {
            return Self::OnePair;
        }

        Self::HighCard
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../assets/seven.txt");

    const SAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_one() {
        let res = part_one(SAMPLE_INPUT);
        assert_eq!(res, 6440)
    }

    #[test]
    fn solution_one() {
        let res = part_one(INPUT);
        dbg!(res);
    }
}
