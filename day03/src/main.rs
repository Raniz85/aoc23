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

/// Get all Numbers in an input grid
fn get_numbers_from_input(input: &Input) -> impl Iterator<Item=Number> + '_ {
    input
        .as_lines()
        .enumerate()
        .flat_map(|(row, line)| Number::parse_row(row, line))
}

/// Get all Symbols in an input grid
fn get_symbols_from_input(input: &Input) -> impl Iterator<Item=Symbol> + '_ {
    input
        .as_lines()
        .enumerate()
        .flat_map(|(row, line)| Symbol::parse_row(row, line))
}

fn get_part_numbers(input: &Input) -> Vec<u32> {
    let input = input.trim_trailing_newlines();
    let symbols = get_symbols_from_input(&input).collect_vec();
    // Find all numbers that are adjacent to at least one symbol
    get_numbers_from_input(&input)
        .filter(|number| symbols.iter().any(|symbol| number.is_adjacent(symbol)))
        .map(|number| number.number)
        .collect_vec()
}

fn get_gear_ratios(input: &Input) -> Vec<u32> {
    let input = input.trim_trailing_newlines();
    let numbers = get_numbers_from_input(&input).collect_vec();
    get_symbols_from_input(&input)
        // Find all * symbols
        .filter(|symbol| symbol.symbol == '*')
        // For each * symbol, find all adjacent Numbers and try to collect them into a (Number, Number) tuple
        // This will only be Some if exactly two Numbers are found and None otherwise
        .filter_map(|symbol| {
            numbers
                .iter()
                .filter(|number| number.is_adjacent(&symbol))
                .collect_tuple()
        })
        // Calculate the gear ratio for each pair of Numbers
        .map(|gears: (&Number, &Number)| gears.0.number * gears.1.number)
        .collect_vec()
}

struct Symbol {
    symbol: char,
    row: usize,
    col: usize,
}

impl Symbol {
    fn parse_row(row: usize, line: &str) -> Vec<Symbol> {
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

    pub fn parse_row(row: usize, line: &str) -> Vec<Number> {
        // Group all characters together with their column and collect into a vec
        let indexed_chars = line.chars()
            .enumerate()
            .collect_vec();
        // Split all chars into consecutive runs of ASCII digits, then parse each group into a number
        indexed_chars.split(|(_col, c)| !c.is_ascii_digit())
            .filter(|number| !number.is_empty())
            .map(|number| {
                let start = number.first().expect("Size already checked").0;
                let end = number.last().expect("Size already checked").0;
                let number = number.iter()
                    .map(|(_col, c)| c)
                    .collect::<String>().parse().expect("Only ascii digits from split");
                Number {
                    number,
                    row,
                    start,
                    end
                }
            })
            .collect_vec()
    }
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
