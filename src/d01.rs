use anyhow::Context;

fn inputs() -> anyhow::Result<Vec<String>> {
    let input_string =
        std::fs::read_to_string("inputs/01_input.txt").context("Error while reading input")?;

    Ok(input_string
        .split('\n')
        .map(|s| s.to_owned())
        .collect::<Vec<_>>())
}

pub fn part_01() -> anyhow::Result<()> {
    todo!()
}

pub fn part_02() -> anyhow::Result<()> {
    todo!()
}