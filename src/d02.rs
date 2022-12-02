use anyhow::Context;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Play {
    Rock = 1,
    Paper = 2,
    Sciss = 3,
}

#[derive(Clone, Copy)]
enum Result {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

fn score1(round: &(Play, Play)) -> u32 {
    let round_res = match round {
        (Play::Rock, Play::Paper) | (Play::Paper, Play::Sciss) | (Play::Sciss, Play::Rock) => {
            Result::Win as u32
        }
        (a, b) if a == b => Result::Draw as u32,
        _ => Result::Lose as u32,
    };

    round_res + round.1 as u32
}

fn score2(round: &(Play, Result)) -> u32 {
    let play_b = match round {
        (play_a, Result::Draw) => *play_a,
        (play_a, Result::Win) => match play_a {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Sciss,
            Play::Sciss => Play::Rock,
        },
        (play_a, Result::Lose) => match play_a {
            Play::Rock => Play::Sciss,
            Play::Paper => Play::Rock,
            Play::Sciss => Play::Paper,
        },
    };

    play_b as u32 + round.1 as u32
}

fn inputs() -> anyhow::Result<String> {
    std::fs::read_to_string("inputs/02_input.txt").context("Error while reading input")
}

fn parse1(input: String) -> anyhow::Result<Vec<(Play, Play)>> {
    Ok(input
        .split('\n')
        .map(|s| {
            let s = s.trim();

            let play_a = match s.chars().nth(0).unwrap() {
                'A' => Play::Rock,
                'B' => Play::Paper,
                'C' => Play::Sciss,
                e => panic!("bad input {}", e),
            };

            let play_b = match s.chars().nth(2).unwrap() {
                'X' => Play::Rock,
                'Y' => Play::Paper,
                'Z' => Play::Sciss,
                e => panic!("bad input {}", e),
            };

            (play_a, play_b)
        })
        .collect())
}

fn parse2(input: String) -> anyhow::Result<Vec<(Play, Result)>> {
    Ok(input
        .split('\n')
        .map(|s| {
            let s = s.trim();

            let play_a = match s.chars().nth(0).unwrap() {
                'A' => Play::Rock,
                'B' => Play::Paper,
                'C' => Play::Sciss,
                e => panic!("bad input {}", e),
            };

            let result = match s.chars().nth(2).unwrap() {
                'X' => Result::Lose,
                'Y' => Result::Draw,
                'Z' => Result::Win,
                e => panic!("bad input {}", e),
            };

            (play_a, result)
        })
        .collect())
}

pub fn part_01() -> anyhow::Result<u32> {
    let input = inputs()?;
    let rounds = parse1(input)?;

    Ok(rounds.iter().map(|round| score1(round)).sum())
}

pub fn part_02() -> anyhow::Result<u32> {
    let input = inputs()?;
    let rounds = parse2(input)?;

    Ok(rounds.iter().map(|round| score2(round)).sum())
}
