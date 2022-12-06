use anyhow::Context;
use itertools::Itertools;
use sliding_windows::IterExt;

fn inputs() -> anyhow::Result<String> {
    let input_string =
        std::fs::read_to_string("inputs/06_input.txt").context("Error while reading input")?;

    Ok(input_string)
}

pub fn part_01() -> anyhow::Result<usize> {
    let input = inputs()?;

    solve(input, 4)
}

pub fn part_02() -> anyhow::Result<usize> {
    let input = inputs()?;

    solve(input, 14)
}

fn solve(input: String, message_len: usize) -> anyhow::Result<usize> {
    let mut window_buffer = sliding_windows::Storage::new(message_len);

    let (ix, _numbers_tuple) = input
        .chars()
        .into_iter()
        .sliding_windows(&mut window_buffer)
        .enumerate()
        .find(|(_, window)| are_all_different(window.iter().map(|c| *c).collect_vec()))
        .context("None found")?;

    Ok(ix + message_len)
}

fn are_all_different(window_vec: Vec<char>) -> bool {
    for ix in 0..(window_vec.len() - 1) {
        for iy in (ix + 1)..window_vec.len() {
            if window_vec[ix] == window_vec[iy] {
                return false;
            }
        }
    }
    return true;
}
