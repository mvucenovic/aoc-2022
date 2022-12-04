use anyhow::Context;
use gcollections::ops::*;
use interval::{interval_set::ToIntervalSet, IntervalSet};
use itertools::Itertools;
use regex::Regex;

fn inputs() -> anyhow::Result<Vec<(IntervalSet<u32>, IntervalSet<u32>)>> {
    let input_string =
        std::fs::read_to_string("inputs/04_input.txt").context("Error while reading input")?;

    parse(input_string)
}

fn parse(input: String) -> anyhow::Result<Vec<(IntervalSet<u32>, IntervalSet<u32>)>> {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    Ok(input
        .split('\n')
        .map(|s| {
            let s = s.trim();

            let caps = re.captures(s).unwrap();

            let nums = caps
                .iter()
                .skip(1)
                .map(|c| c.unwrap().as_str().parse::<u32>().unwrap())
                .collect_vec();

            let first_elf = (nums[0], nums[1]).to_interval_set();
            let second_elf = (nums[2], nums[3]).to_interval_set();
            (first_elf, second_elf)
        })
        .collect())
}

pub fn part_01() -> anyhow::Result<usize> {
    let input = inputs()?;

    Ok(input
        .into_iter()
        .map(|(first_elf, second_elf)| {
            first_elf.is_subset(&second_elf) || second_elf.is_subset(&first_elf)
        })
        .filter(|contains_other| *contains_other)
        .count())
}

pub fn part_02() -> anyhow::Result<usize> {
    let input = inputs()?;

    Ok(input
        .into_iter()
        .map(|(first_elf, second_elf)| !first_elf.is_disjoint(&second_elf))
        .filter(|have_some_overlap| *have_some_overlap)
        .count())
}
