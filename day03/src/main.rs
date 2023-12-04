use anyhow::Result;
use itertools::Itertools;

use util::Input;

fn main() -> Result<()> {
    let input = Input::load("day03/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &Input) -> Result<u32> {
    Ok(get_part_numbers(input).into_iter().sum())
}

fn part2(input: &Input) -> Result<u32> {
    Ok(get_gear_ratios(input).into_iter().sum())
}

fn get_part_numbers(input: &Input) -> Vec<u32> {
    let input = input.trim_trailing_newlines();
    let symbols = input
        .as_lines()
        .enumerate()
        .flat_map(|(row, line)| get_symbols(row, line))
        .collect_vec();
    input
        .as_lines()
        .enumerate()
        .flat_map(|(row, line)| get_numbers(row, line))
        .filter(|number| symbols.iter().any(|symbol| number.is_adjacent(symbol)))
        .map(|number| number.number)
        .collect_vec()
}

fn get_gear_ratios(input: &Input) -> Vec<u32> {
    let input = input.trim_trailing_newlines();
    let numbers = input
        .as_lines()
        .enumerate()
        .flat_map(|(row, line)| get_numbers(row, line))
        .collect_vec();
    input
        .as_lines()
        .enumerate()
        .flat_map(|(row, line)| get_symbols(row, line))
        .filter(|symbol| symbol.symbol == '*')
        .filter_map(|symbol| {
            numbers
                .iter()
                .filter(|number| number.is_adjacent(&symbol))
                .collect_tuple()
        })
        .map(|gears: (&Number, &Number)| gears.0.number * gears.1.number)
        .collect_vec()
}

struct Symbol {
    symbol: char,
    row: usize,
    col: usize,
}

fn get_symbols(row: usize, line: &str) -> Vec<Symbol> {
    line.chars()
        .enumerate()
        .filter(|(_col, c)| !c.is_ascii_digit() && *c != '.')
        .map(|(col, c)| Symbol {
            symbol: c,
            row,
            col,
        })
        .collect_vec()
}

struct Number {
    number: u32,
    row: usize,
    start: usize,
    end: usize,
}

impl Number {
    pub fn is_adjacent(&self, symbol: &Symbol) -> bool {
        self.row.abs_diff(symbol.row) <= 1
            && self.start.saturating_sub(1) <= symbol.col
            && symbol.col <= self.end.saturating_add(1)
    }
}

fn get_numbers(row: usize, line: &str) -> Vec<Number> {
    line.chars()
        .chain(['.'])
        .enumerate()
        .fold(
            (Vec::new(), String::new(), 0),
            |(mut numbers, mut current_number, start), (col, c)| {
                if c.is_ascii_digit() {
                    let start = if current_number.is_empty() {
                        col
                    } else {
                        start
                    };
                    current_number.push(c);
                    (numbers, current_number, start)
                } else if current_number.is_empty() {
                    (numbers, current_number, col)
                } else {
                    let number = current_number
                        .parse()
                        .expect("Contents of string should be vetted with char::is_ascii_digit()");
                    numbers.push(Number {
                        number,
                        row,
                        start,
                        end: col - 1,
                    });
                    (numbers, String::new(), col)
                }
            },
        )
        .0
}

#[cfg(test)]
mod test {
    use crate::{get_gear_ratios, get_part_numbers, part1, part2, Number, Symbol};
    use anyhow::Result;
    use rstest::rstest;
    use util::Input;

    #[rstest]
    #[case(2, 4)]
    #[case(2, 7)]
    #[case(2, 6)]
    #[case(1, 3)]
    #[case(3, 3)]
    #[case(1, 7)]
    #[case(3, 7)]
    #[case(3, 5)]
    #[case(1, 5)]
    pub fn test_is_adjacent(#[case] symbol_row: usize, #[case] symbol_col: usize) {
        // Given a number
        let number = Number {
            number: 1,
            row: 2,
            start: 4,
            end: 6,
        };

        // and a symbol
        let symbol = Symbol {
            symbol: '*',
            row: symbol_row,
            col: symbol_col,
        };

        // Expect them to be adjacent
        assert!(number.is_adjacent(&symbol));
    }

    #[test]
    pub fn test_is_adjacent_starts_at_zero() {
        // Given a number
        let number = Number {
            number: 1,
            row: 0,
            start: 0,
            end: 6,
        };

        // and a symbol
        let symbol = Symbol {
            symbol: '*',
            row: 1,
            col: 1,
        };

        // Expect them to be adjacent
        assert!(number.is_adjacent(&symbol));
    }

    #[rstest]
    #[case(2, 2)]
    #[case(1, 2)]
    #[case(0, 4)]
    #[case(4, 4)]
    pub fn test_is_not_adjacent(#[case] symbol_row: usize, #[case] symbol_col: usize) {
        // Given a number
        let number = Number {
            number: 1,
            row: 2,
            start: 4,
            end: 6,
        };

        // and a symbol
        let symbol = Symbol {
            symbol: '*',
            row: symbol_row,
            col: symbol_col,
        };

        // Expect them to be adjacent
        assert!(!number.is_adjacent(&symbol));
    }

    #[test]
    pub fn test_get_part_numbers() {
        // Given some input
        let input = Input::from_lines([
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
            "......+321",
        ]);

        // When the part numbers are extracted
        let numbers = get_part_numbers(&input);

        // Then they are as expected
        assert_eq!(numbers, vec![467, 35, 633, 617, 592, 755, 664, 598, 321])
    }

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]);
        assert_eq!(part1(&input).unwrap(), 4361);
        Ok(())
    }

    #[test]
    pub fn test_get_gear_ratios() {
        // Given som input
        let input = Input::from_lines([
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]);

        // When the gear ratios is retrieved
        let ratios = get_gear_ratios(&input);

        // Then they are as expected
        assert_eq!(ratios, vec![16345, 451490])
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines([
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]);
        assert_eq!(part2(&input).unwrap(), 467835);
        Ok(())
    }
}
