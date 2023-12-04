use anyhow::{anyhow, bail, Result};
use itertools::Itertools;

use util::Input;

fn main() -> Result<()> {
    let input = Input::load("day01/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &Input) -> Result<u32> {
    input
        .trim_trailing_newlines()
        .as_lines()
        .map(get_calibration_number)
        .map_ok(|nbr| nbr as u32)
        .sum()
}

fn part2(input: &Input) -> Result<u32> {
    input
        .trim_trailing_newlines()
        .as_lines()
        .map(get_calibration_number_spelled_out)
        .map_ok(|nbr| nbr as u32)
        .sum()
}

fn get_calibration_number(input: &str) -> Result<u8> {
    let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
    let input = match digits.len() {
        0 => bail!("Erroneous input"),
        1 => format!("{}{}", digits, digits),
        2 => digits,
        len => format!(
            "{}{}",
            digits
                .chars()
                .next()
                .expect("Iterator has length > 2 according to match"),
            digits
                .chars()
                .nth(len - 1)
                .expect("Iterator has length > 2 according to match"),
        ),
    };
    Ok(input.parse()?)
}

static NUMBERS: [(&str, u8); 20] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn get_calibration_number_spelled_out(input: &str) -> Result<u8> {
    // Find the first and last occurrences of each digit or word
    // and sort them according to where in the string they occur
    let digits = NUMBERS
        .iter()
        .flat_map(|(word, value)| {
            [
                input.find(word).map(|position| (position, *value)),
                input.rfind(word).map(|position| (position, *value)),
            ]
        })
        .flatten()
        .sorted()
        .collect_vec();

    // Get the first and last digits
    let first = digits.first();
    let last = digits.last();
    match (first, last) {
        // These are either always Some, Some or None, None
        (Some(first), Some(last)) => Ok(10 * first.1 + last.1),
        _ => Err(anyhow!("Invalid input")),
    }
}

#[cfg(test)]
mod test {
    use crate::{get_calibration_number, get_calibration_number_spelled_out, part1, part2};
    use anyhow::Result;
    use rstest::rstest;
    use util::Input;

    #[rstest]
    #[case("12", 12)]
    #[case("11", 11)]
    #[case("1", 11)]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    pub fn that_get_calibration_number_returns_correct_calibration_number(
        #[case] input: &str,
        #[case] expected: u8,
    ) {
        // When the calibration number is extracted
        let nbr = get_calibration_number(input);

        // Then it is as expected
        assert_eq!(expected, nbr.unwrap());
    }

    #[rstest]
    #[case("12", 12)]
    #[case("11", 11)]
    #[case("1", 11)]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("7pqrsteighthree", 73)]
    #[case("7237", 77)]
    pub fn that_get_calibration_number_spelled_out_returns_correct_calibration_number(
        #[case] input: &str,
        #[case] expected: u8,
    ) {
        // When the calibration number is extracted
        let nbr = get_calibration_number_spelled_out(input);

        // Then it is as expected
        assert_eq!(expected, nbr.unwrap());
    }

    #[rstest]
    #[case("")]
    #[case("abcdef")]
    pub fn that_get_calibration_number_for_erroneous_input_returns_err(#[case] input: &str) {
        // When the calibration number is extracted
        let result = get_calibration_number(input);

        // Then it is an error
        assert!(result.is_err());
    }

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines(["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]);
        assert_eq!(part1(&input).unwrap(), 142);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines([
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]);
        assert_eq!(part2(&input).unwrap(), 281);
        Ok(())
    }
}
