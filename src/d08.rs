use anyhow::Context;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn inputs() -> anyhow::Result<Vec<Vec<u32>>> {
    let input_string =
        std::fs::read_to_string("inputs/08_input.txt").context("Error while reading input")?;

    parse(input_string)
}

fn parse(input: String) -> anyhow::Result<Vec<Vec<u32>>> {
    Ok(input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect_vec())
}

pub fn part_01() -> anyhow::Result<u32> {
    let forest = inputs()?;
    let (dim_x, dim_y) = (forest[0].len(), forest.len());

    let mut visible = 0;
    for ix in 0..dim_x {
        for iy in 0..dim_y {
            if is_visible(&forest, (ix, iy)) {
                visible += 1
            }
        }
    }

    Ok(visible)
}

fn is_visible(forest: &Vec<Vec<u32>>, coord: (usize, usize)) -> bool {
    let (x, y) = coord;
    let (dim_x, dim_y) = (forest[0].len(), forest.len());
    let tree = forest[y][x];

    (0..x).all(|ix| forest[y][ix] < tree)
        || (x + 1..dim_x).all(|ix| forest[y][ix] < tree)
        || (0..y).all(|iy| forest[iy][x] < tree)
        || (y + 1..dim_y).all(|iy| forest[iy][x] < tree)
}

pub fn part_02() -> anyhow::Result<usize> {
    let forest = inputs()?;
    let (dim_x, dim_y) = (forest[0].len(), forest.len());

    let mut max_scenic = 0;
    for ix in 0..dim_x {
        for iy in 0..dim_y {
            let cur = scenic_score(&forest, (ix, iy));
            max_scenic = std::cmp::max(cur, max_scenic);
        }
    }

    Ok(max_scenic)
}

fn scenic_score(forest: &Vec<Vec<u32>>, coord: (usize, usize)) -> usize {
    let (x, y) = coord;
    let (dim_x, dim_y) = (forest[0].len(), forest.len());
    let tree = forest[y][x];

    let acc_x = |acc, ix| {
        if forest[y][ix] < tree {
            Continue(acc + 1)
        } else {
            Done(acc + 1)
        }
    };

    let left = (0..x).rev().fold_while(0, acc_x).into_inner();
    let right = (x + 1..dim_x).fold_while(0, acc_x).into_inner();

    let acc_y = |acc, iy: usize| {
        if forest[iy][x] < tree {
            Continue(acc + 1)
        } else {
            Done(acc + 1)
        }
    };

    let up = (0..y).rev().fold_while(0, acc_y).into_inner();
    let down = (y + 1..dim_y).fold_while(0, acc_y).into_inner();

    left * right * up * down
}
