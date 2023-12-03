fn part_one(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();

        let value: i32 = format!("{}{}", first, last).parse().unwrap();
        sum += value;
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../assets/one.txt");

    #[test]
    fn test_one() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let res = part_one(input);

        assert_eq!(res, 142)
    }

    #[test]
    fn solution_one() {
        let res = part_one(INPUT);
        dbg!(res);
    }
}
