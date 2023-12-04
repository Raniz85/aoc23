use std::collections::HashSet;

use std::str::FromStr;
use anyhow::{anyhow, Result};
use itertools::Itertools;

use util::Input;

fn main() -> Result<()> {
    let input = Input::load("day04/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}


fn part1(input: &Input) -> Result<u64> {
    input.trim_trailing_newlines().as_lines()
        .map(Card::from_str)
        .map_ok(|card| card.score())
        .sum()
}

fn part2(input: &Input) -> Result<u64> {
    // All the cards that we start with
    let cards: Vec<Card> =
        input.trim_trailing_newlines().as_lines()
            .map(Card::from_str)
            .try_collect()?;
    // Vector to keep track of how many we have of each card
    let mut card_counts = vec![1; cards.len()];

    // Go through each card, adding copies of each card that comes after if we win
    for (index, card) in cards.iter().enumerate() {
        let matches = card.matches();
        let next = index +1;
        let last = (index + 1 + matches).min(cards.len());

        // Add the number of instances of this card to each following card
        // i.e. 2 copies of card 2 with 2 matches adds 2 more copies of card 3 and 4
        for add_index in next..last {
            card_counts[add_index] += card_counts[index];
        }
    }
    // Sum the number of cards we have
    let sum: usize = card_counts.iter().sum();
    Ok(sum as u64)
}

struct Card {
    winners: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl Card {

    /// Calculate the number of matches for this card
    pub fn matches(&self) -> usize {
        self.winners.intersection(&self.numbers).count()
    }

    /// Calculate the score for this card
    pub fn score(&self) -> u64 {
        match self.matches() {
            0 => 0,
            matches => 2u64.pow(matches as u32 - 1),
        }
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (_declaration, winners, numbers) = s.split(&['|', ':'])
            .collect_tuple().ok_or_else(|| anyhow!("Invalid card: `{}`", s))?;
        let winners: HashSet<u32> = winners.split(' ')
            .filter_map(|n| Some(n.trim()).filter(|n| !n.is_empty()).map(|n| n.parse()))
            .try_collect()?;
        let numbers: HashSet<u32> = numbers.trim().split(' ')
            .filter_map(|n| Some(n.trim()).filter(|n| !n.is_empty()).map(|n| n.parse()))
            .try_collect()?;
        Ok(Card {
            winners,
            numbers,
        })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::{Card, part1, part2};
    use anyhow::Result;
    use rstest::rstest;
    use util::Input;

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    pub fn test_get_card_score(#[case] card: &str, #[case] expected_score: u64) {
        // Given a card
        let card = Card::from_str(card).unwrap();

        // Expect the cards score to be correct
        assert_eq!(card.score(), expected_score);
    }

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]);
        assert_eq!(part1(&input).unwrap(), 13);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines([
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]);
        assert_eq!(part2(&input).unwrap(), 30);
        Ok(())
    }
}
