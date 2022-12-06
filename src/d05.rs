use anyhow::Context;
use regex::Regex;
use std::collections::VecDeque;

fn inputs() -> anyhow::Result<(Vec<VecDeque<char>>, Vec<(usize, usize, usize)>)> {
    let input_string =
        std::fs::read_to_string("inputs/05_input.txt").context("Error while reading input")?;

    parse(input_string)
}

fn parse(input: String) -> anyhow::Result<(Vec<VecDeque<char>>, Vec<(usize, usize, usize)>)> {
    let (crates_str, moves_str) = input.split_once("\n\n").unwrap();

    let crates = parse_crates(crates_str);

    let moves = parse_moves(moves_str)?;

    Ok((crates, moves))
}

fn parse_crates(crates_str: &str) -> Vec<VecDeque<char>> {
    let mut crates = vec![VecDeque::new(); 9];

    for line in crates_str.lines().rev().skip(1) {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, ch)| *ch != ' ')
            .for_each(|(ix, ch)| crates[ix].push_back(ch));
    }

    crates
}

fn parse_moves(moves_str: &str) -> anyhow::Result<Vec<(usize, usize, usize)>> {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let mut moves = vec![];
    for line in moves_str.lines() {
        let caps = re.captures(line).context("bad moves input")?;

        let cnt = caps[1].parse::<usize>()?;
        let from = caps[2].parse::<usize>()? - 1; // -1 since we are not savages and index from 0!
        let to = caps[3].parse::<usize>()? - 1;
        moves.push((cnt, from, to));
    }
    Ok(moves)
}

pub fn part_01() -> anyhow::Result<String> {
    let (mut crates, moves) = inputs()?;

    for (cnt, from, to) in moves {
        let split_point = crates[from].len() - cnt;
        let crates_to_move = crates[from].split_off(split_point);
        crates[to].extend(crates_to_move.into_iter().rev());
    }

    Ok(crates
        .into_iter()
        .map(|mut stack| stack.pop_back().unwrap_or(' '))
        .collect::<String>())
}

pub fn part_02() -> anyhow::Result<String> {
    let (mut crates, moves) = inputs()?;

    for (cnt, from, to) in moves {
        let split_point = crates[from].len() - cnt;
        let crates_to_move = crates[from].split_off(split_point);
        crates[to].extend(crates_to_move);
    }

    Ok(crates
        .into_iter()
        .map(|mut stack| stack.pop_back().unwrap_or(' '))
        .collect::<String>())
}
