use anyhow::{anyhow, Result};

use util::Input;

fn main() -> Result<()> {
    let input = Input::load("dayXX/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}


fn part1(input: &Input) -> Result<u32> {
    Ok(0)
}

fn part2(input: &Input) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    /*
    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
        ]);
        assert_eq!(part1(&input).unwrap(), 0);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines([
        ]);
        assert_eq!(part2(&input).unwrap(), 0);
        Ok(())
    }
     */
}
