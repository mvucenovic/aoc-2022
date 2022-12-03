use anyhow::Context;
use itertools::Itertools;
use std::collections::HashSet;

fn inputs() -> anyhow::Result<Vec<(Vec<char>, Vec<char>)>> {
    let input_string =
        std::fs::read_to_string("inputs/03_input.txt").context("Error while reading input")?;

    parse(input_string)
}

fn parse(input: String) -> anyhow::Result<Vec<(Vec<char>, Vec<char>)>> {
    Ok(input
        .split('\n')
        .map(|s| {
            let s = s.trim();
            let middle = s.len() / 2;
            let (first, second) = s.split_at(middle);
            (first.chars().collect_vec(), second.chars().collect_vec())
        })
        .collect())
}

pub fn part_01() -> anyhow::Result<u32> {
    let sacks = inputs()?;

    Ok(sacks.into_iter().map(one_sack_value).sum())
}

fn one_sack_value(sack: (Vec<char>, Vec<char>)) -> u32 {
    let (first, second) = sack;
    let first_set = first.into_iter().collect::<HashSet<char>>();
    let second_set = second.into_iter().collect::<HashSet<char>>();

    first_set
        .intersection(&second_set)
        .map(|ch| value(ch))
        .sum::<u32>()
}

fn value(ch: &char) -> u32 {
    if ch.is_uppercase() {
        *ch as u32 - 'A' as u32 + 27
    } else {
        *ch as u32 - 'a' as u32 + 1
    }
}

pub fn part_02() -> anyhow::Result<u32> {
    let sacks = inputs()?;

    Ok(sacks
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|group| {
            let sets = group
                .into_iter()
                .map(|(mut f, s)| {
                    f.extend(s);
                    f.into_iter().collect::<HashSet<char>>()
                })
                .collect_vec();

            let first_inter = sets[0]
                .intersection(&sets[1])
                .map(|ch| ch.to_owned())
                .collect::<HashSet<_>>();
            sets[2]
                .intersection(&first_inter)
                .map(|ch| value(ch))
                .sum::<u32>()
        })
        .sum())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_one_sack_value() {
        let sack = (
            "vJrwpWtwJgWr".chars().collect_vec(),
            "hcsFMMfFFhFp".chars().collect_vec(),
        );
        let value = one_sack_value(sack);
        assert_eq!(value, 16);

        let sack = (
            "PmmdzqPrV".chars().collect_vec(),
            "vPwwTWBwg".chars().collect_vec(),
        );
        let value = one_sack_value(sack);
        assert_eq!(value, 42);
    }
}
