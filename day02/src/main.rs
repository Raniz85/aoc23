use anyhow::{anyhow, Result};
use itertools::Itertools;

use util::Input;

fn main() -> Result<()> {
    let input = Input::load("day02/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Game {
    id: u32,
    hands: Vec<Hand>,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy, Default)]
pub struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

impl Hand {
    pub fn new(red: u32, green: u32, blue: u32) -> Hand {
        Hand { red, green, blue }
    }

    pub fn parse(input: &str) -> Result<Hand> {
        let hand = input
            .split(',')
            .map(|cube| {
                let (count, colour) = cube
                    .trim()
                    .splitn(2, ' ')
                    .map(str::trim)
                    .collect_tuple()
                    .ok_or_else(|| anyhow!("Invalid cube declaration {}", cube))?;
                let count: u32 = count.parse()?;
                match colour {
                    "red" | "green" | "blue" => Ok((colour, count)),
                    other => Err(anyhow!("Illegal colour {}", other)),
                }
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .fold(Hand::default(), |hand, (colour, count)| match colour {
                "red" => Hand {
                    red: count,
                    green: hand.green,
                    blue: hand.blue,
                },
                "green" => Hand {
                    red: hand.red,
                    green: count,
                    blue: hand.blue,
                },
                "blue" => Hand {
                    red: hand.red,
                    green: hand.green,
                    blue: count,
                },
                other => unreachable!("Invalid colour {} slipped through", other),
            });
        Ok(hand)
    }

    pub fn is_valid(&self, limits: &Hand) -> bool {
        self.red <= limits.red && self.green <= limits.green && self.blue <= limits.blue
    }

    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl Game {
    pub fn parse_many(input: &Input) -> Result<Vec<Game>> {
        input
            .trim_trailing_newlines()
            .as_lines()
            .map(Game::parse)
            .collect::<Result<Vec<_>>>()
    }

    pub fn parse(input: &str) -> Result<Game> {
        let (declaration, cubes) = input
            .splitn(2, ':')
            .collect_tuple()
            .ok_or_else(|| anyhow!("Invalid game {}", input))?;
        let (_, id) = declaration
            .splitn(2, ' ')
            .collect_tuple()
            .ok_or_else(|| anyhow!("Invalid game ID {}", declaration))?;
        let id = id.parse()?;
        let hands = cubes
            .split(';')
            .map(Hand::parse)
            .collect::<Result<Vec<_>>>()?;
        Ok(Game { id, hands })
    }

    pub fn is_valid(&self, limits: &Hand) -> bool {
        self.hands.iter().all(|hand| hand.is_valid(limits))
    }

    pub fn power(&self) -> u32 {
        self.hands
            .iter()
            .fold(Hand::default(), |maximums, hand| Hand {
                red: maximums.red.max(hand.red),
                green: maximums.green.max(hand.green),
                blue: maximums.blue.max(hand.blue),
            })
            .power()
    }
}

fn part1(input: &Input) -> Result<u32> {
    let limits = Hand::new(12, 13, 14);
    let id_sum = Game::parse_many(input)?
        .into_iter()
        .filter_map(|game| {
            if game.is_valid(&limits) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum();
    Ok(id_sum)
}

fn part2(input: &Input) -> Result<u32> {
    let total_power = Game::parse_many(input)?.iter().map(Game::power).sum();
    Ok(total_power)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, Game, Hand};
    use anyhow::Result;
    use rstest::rstest;
    use util::Input;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Game {
        id: 1,
        hands: vec![
            Hand::new(4, 0, 3), Hand::new(1, 2, 6), Hand::new(0, 2, 0)
        ]
    })]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", Game {
        id: 2,
        hands: vec![
            Hand::new(0, 2, 1), Hand::new(1, 3, 4), Hand::new(0, 1, 1)
        ]
    })]
    pub fn test_parse_game_parses_correctly(#[case] input: &str, #[case] expected: Game) {
        // When the input is parsed
        let parsed = Game::parse(input);

        // Then the returned game is as expected
        assert_eq!(parsed.unwrap(), expected)
    }

    #[rstest]
    #[case(Game {
            id: 1,
            hands: vec![
                Hand::new(4, 0, 3), Hand::new(1, 2, 6), Hand::new(0, 2, 0)
            ]
        },
        4 * 2 * 6,
    )]
    #[case(Game {
            id: 2,
            hands: vec![
                Hand::new(0, 2, 1), Hand::new(1, 3, 4), Hand::new(0, 1, 1)
            ]
        },
        3 * 4,
    )]
    pub fn test_game_power_is_calculated_correctly(#[case] game: Game, #[case] expected: u32) {
        // When the power of the game is retrieved
        let power = game.power();

        // Then the returned game is as expected
        assert_eq!(power, expected)
    }

    #[rstest]
    #[case(
        Game {
            id: 1,
            hands: vec![
                Hand::new(4, 0, 3), Hand::new(1, 2, 6), Hand::new(0, 2, 0)
            ],
        },
        Hand::new(12, 13, 14),
        true,
    )]
    #[case(
        Game {
            id: 3,
            hands: vec![
                Hand::new(20, 8, 6), Hand::new(4, 13, 5), Hand::new(1, 5, 0)
            ],
        },
        Hand::new(12, 13, 14),
        false,
    )]
    pub fn test_is_game_valid(#[case] game: Game, #[case] limits: Hand, #[case] expected: bool) {
        // When validity of the game is checked
        let valid = game.is_valid(&limits);

        // Then the validity is as expected
        assert_eq!(expected, valid);
    }

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]);
        assert_eq!(part1(&input).unwrap(), 8);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines([
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]);
        assert_eq!(part2(&input).unwrap(), 2286);
        Ok(())
    }
}
