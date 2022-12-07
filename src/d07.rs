use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;

fn inputs() -> anyhow::Result<String> {
    let input_string =
        std::fs::read_to_string("inputs/07_input.txt").context("Error while reading input")?;

    Ok(input_string)
}

pub fn part_01() -> anyhow::Result<usize> {
    let terminal = inputs()?;

    let dir_sizes = dir_sizes(terminal)?;

    let res: usize = dir_sizes
        .into_iter()
        .filter(|(_, v)| *v <= 100_000)
        .map(|(_, v)| v)
        .sum();

    Ok(res)
}

pub fn part_02() -> anyhow::Result<usize> {
    let terminal = inputs()?;

    let disk_size = 70_000_000;
    let space_needed = 30_000_000;

    let dir_sizes = dir_sizes(terminal)?;

    let sum_space = *dir_sizes.get("/").context("root should exist")?;
    let currently_available = disk_size - sum_space;
    let minimum_to_delete = space_needed - currently_available;

    let dir_to_delete_size = dir_sizes
        .into_iter()
        .filter(|(_, v)| *v >= minimum_to_delete)
        .map(|(_, v)| v)
        .min();

    Ok(dir_to_delete_size.unwrap())
}

fn dir_sizes(terminal: String) -> anyhow::Result<HashMap<String, usize>> {
    let lines = terminal.lines();

    let mut dirs: HashMap<String, usize> = HashMap::new();
    let mut cwd = PathBuf::from("/");

    for line in lines {
        let line = line.trim();
        match line.chars().next().unwrap() {
            '$' => {
                if line.starts_with("$ cd ") {
                    // this is a `cd` command
                    let (_, cd) = line.split_at("$ cd ".len());
                    if cd == "/" {
                        cwd = PathBuf::from("/");
                    } else if cd == ".." {
                        cwd.pop();
                    } else {
                        cwd.push(cd);
                    }
                }
            }
            '1'..='9' => {
                // this is a file
                let file_size = line.split(" ").next().unwrap().parse::<usize>()?;

                let mut paths = cwd.clone();
                loop {
                    let path = paths.to_string_lossy().to_string();
                    *dirs.entry(path).or_insert(0) += file_size;
                    let has_more = paths.pop();
                    if !has_more {
                        break;
                    }
                }
            }
            _ => {}
        }
    }
    Ok(dirs)
}
