use std::collections::HashMap;

use anyhow::Context;
use itertools::Itertools;

// 14000 too high

fn inputs() -> anyhow::Result<Vec<Instruction>> {
    let input_string =
        std::fs::read_to_string("inputs/10_input.txt").context("Error while reading input")?;

    let input_string2 = "addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop"
        .to_owned();

    parse(input_string)
}

enum Instruction {
    Addx(i32),
    Noop,
}

fn parse(input: String) -> anyhow::Result<Vec<Instruction>> {
    Ok(input
        .lines()
        .map(|line| {
            let line = line.trim();
            if line.starts_with("addx") {
                Instruction::Addx(line.split_once(' ').unwrap().1.parse().unwrap())
            } else if line == "noop" {
                Instruction::Noop
            } else {
                panic!("bad input")
            }
        })
        .collect())
}

pub fn part_01() -> anyhow::Result<i32> {
    let program = inputs()?;

    let mut state_map = HashMap::new();
    let mut cycle = 1;
    let mut reg_x = 1;
    state_map.insert(1, 1);
    for instruction in program {
        cycle += 1;
        state_map.insert(cycle, reg_x);
        match instruction {
            Instruction::Addx(val) => {
                cycle += 1;
                reg_x += val;
                state_map.insert(cycle, reg_x);
            }
            Instruction::Noop => {}
        }
    }

    let v = [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|cycle| state_map.get(&cycle).unwrap_or(&0) * cycle)
        .collect_vec();
    println!(
        "{:#?}",
        state_map
            .into_iter()
            .sorted_by_key(|(k, v)| *k)
            .collect_vec()
    );
    Ok(v.into_iter().sum())
}

pub fn part_02() -> anyhow::Result<()> {
    let program = inputs()?;

    let mut state_map = HashMap::new();
    state_map.insert(1, 1);
    let mut cycle = 1;
    let mut reg_x = 1;
    for instruction in program {
        cycle += 1;
        state_map.insert(cycle, reg_x);
        match instruction {
            Instruction::Addx(val) => {
                cycle += 1;
                reg_x += val;
                state_map.insert(cycle, reg_x);
            }
            Instruction::Noop => {}
        }
    }

    let mut buff = String::new();
    for pixel in 1..=240 {
        // println!("{}", pixel);
        let x = *state_map.get(&pixel).unwrap();
        if is_pixel_visible(pixel, x) {
            buff.push('#')
        } else {
            buff.push('.')
        };

        if pixel % 40 == 0 {
            println!("{}", buff);
            buff.clear();
        }
    }
    Ok(())
}

fn is_pixel_visible(cycle: i32, x: i32) -> bool {
    cycle % 40 - 1 >= x - 1 && cycle % 40 - 1 <= x + 1
}
