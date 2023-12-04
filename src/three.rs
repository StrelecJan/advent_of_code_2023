use std::collections::HashMap;

fn part_one(input: &str) -> i32 {
    let (symbols, values) = parse_input(input);

    let values: Vec<i32> = values
        .iter()
        .filter(|v| v.is_adjacent(&symbols))
        .map(|v| v.value())
        .collect();

    values.iter().sum()
}

fn parse_input(input: &str) -> (Symbols, Vec<Value>) {
    let mut values = vec![];
    let mut symbols = Symbols::new();

    for (y, line) in input.lines().enumerate() {
        let mut value = Value::new();

        for (x, c) in line.chars().enumerate() {
            // Add digit to number
            if c.is_ascii_digit() {
                let number = c.to_digit(10).unwrap() as i32;
                let number = Digit {
                    value: number,
                    coordinate: Coordinate::new(x as isize, y as isize),
                };
                value.add(number);
                continue;
            }

            if c == '.' {
                if !value.is_new() {
                    values.push(value);
                    value = Value::new();
                }
                continue;
            }

            if !value.is_new() {
                values.push(value);
                value = Value::new();
            }
            symbols.insert(
                Coordinate::new(x as isize, y as isize),
                Symbol {
                    symbol: c,
                    part_numbers: vec![],
                },
            );
        }

        if !value.is_new() {
            values.push(value);
        }
    }

    (symbols, values)
}

#[derive(Debug)]
struct Value {
    digits: Vec<Digit>,
}

impl Value {
    pub fn new() -> Self {
        Self { digits: vec![] }
    }

    pub fn add(&mut self, number: Digit) {
        self.digits.push(number);
    }

    pub fn is_new(&self) -> bool {
        self.digits.is_empty()
    }

    pub fn is_adjacent(&self, symbols: &Symbols) -> bool {
        for number in &self.digits {
            if number.coordinate.is_adjacent(symbols) {
                return true;
            }
        }

        false
    }

    pub fn value(&self) -> i32 {
        let chars: String = self
            .digits
            .iter()
            .map(|d| char::from_digit(d.value as u32, 10).unwrap())
            .collect();

        chars.parse().unwrap()
    }
}

#[derive(Debug)]
struct Digit {
    value: i32,
    coordinate: Coordinate,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn is_adjacent(&self, symbols: &Symbols) -> bool {
        let x = self.x;
        let y = self.y;

        // Vertical
        let top = symbols.contains_key(&Coordinate::new(x, y - 1));
        let bottom = symbols.contains_key(&Coordinate::new(x, y + 1));

        // Horizontal
        let left = symbols.contains_key(&Coordinate::new(x - 1, y));
        let right = symbols.contains_key(&Coordinate::new(x + 1, y));

        // Diagonal
        let top_left = symbols.contains_key(&Coordinate::new(x - 1, y - 1));
        let bottom_left = symbols.contains_key(&Coordinate::new(x - 1, y + 1));

        let top_right = symbols.contains_key(&Coordinate::new(x + 1, y - 1));
        let bottom_right = symbols.contains_key(&Coordinate::new(x + 1, y + 1));

        top || bottom || left || right || top_left || bottom_left || top_right || bottom_right
    }
}

struct Symbol {
    symbol: char,
    part_numbers: Vec<i32>,
}
type Symbols = HashMap<Coordinate, Symbol>;

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../assets/three.txt");

    #[test]
    fn test_one() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part_one(input), 4361);
    }

    #[test]
    fn solution_one() {
        let result = part_one(INPUT);
        dbg!(result);
    }
}
