use anyhow::Context;
use itertools::Itertools;

fn inputs() -> anyhow::Result<Vec<Vec<u32>>> {
    let input_string =
        std::fs::read_to_string("inputs/01_input.txt").context("Error while reading input")?;

    parse(input_string)
}

fn parse(input: String) -> anyhow::Result<Vec<Vec<u32>>> {
    let lines = input.split('\n').map(|s| s.to_owned());
    let mut elves = vec![];
    let mut current_elf = vec![];
    for line in lines {
        let line = line.trim();
        if line == "" {
            elves.push(current_elf);
            current_elf = vec![];
        } else {
            current_elf.push(line.parse::<u32>()?)
        }
    }
    if current_elf.len() > 0 {
        elves.push(current_elf);
    }
    Ok(elves)
}

pub fn part_01() -> anyhow::Result<u32> {
    let elves = inputs()?;

    Ok(elves.iter().map(|e| e.iter().sum()).max().unwrap())
}

pub fn part_02() -> anyhow::Result<u32> {
    let elves = inputs()?;

    let tree_elves = elves
        .into_iter()
        .map(|e| e.iter().sum())
        .sorted_by(|a: &u32, b| b.cmp(a))
        .take(3)
        .sum::<u32>();

    Ok(tree_elves)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let input = "1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000";

        let parsed = parse(input.to_string()).unwrap();

        let expected = vec![
            vec![1000u32, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        assert_eq!(expected, parsed)
    }
}
