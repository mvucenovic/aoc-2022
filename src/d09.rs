use anyhow::Context;
use itertools::Itertools;

fn inputs() -> anyhow::Result<Vec<Vec<u32>>> {
    let input_string =
        std::fs::read_to_string("inputs/09_input.txt").context("Error while reading input")?;

    parse(input_string)
}

fn parse(input: String) -> anyhow::Result<Vec<Vec<u32>>> {
    todo!()
}

pub fn part_01() -> anyhow::Result<u32> {
    todo!()
}

pub fn part_02() -> anyhow::Result<u32> {
    todo!()
}
