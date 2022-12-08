use anyhow::Context;
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

    if x == 0 || x == dim_x - 1 || y == 0 || y == dim_y - 1 {
        return true;
    }

    let mut visible_x = true;
    let mut visible_y = true;

    for ix in 0..x {
        if forest[y][ix] >= tree {
            visible_x = false;
            break;
        }
    }

    if !visible_x {
        visible_x = true;
        for ix in (x + 1)..dim_x {
            if forest[y][ix] >= tree {
                visible_x = false;
                break;
            }
        }
    }

    for iy in 0..y {
        if forest[iy][x] >= tree {
            visible_y = false;
            break;
        }
    }

    if !visible_y {
        visible_y = true;
        for iy in (y + 1)..dim_y {
            if forest[iy][x] >= tree {
                visible_y = false;
                break;
            }
        }
    }

    visible_x || visible_y
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

    let mut left = 0;
    for ix in (0..x).rev() {
        left += 1;
        if forest[y][ix] >= tree {
            break;
        }
    }

    let mut right = 0;
    for ix in (x + 1)..dim_x {
        right += 1;
        if forest[y][ix] >= tree {
            break;
        }
    }

    let mut up = 0;
    for iy in (0..y).rev() {
        up += 1;
        if forest[iy][x] >= tree {
            break;
        }
    }

    let mut down = 0;
    for iy in (y + 1)..dim_y {
        down += 1;
        if forest[iy][x] >= tree {
            break;
        }
    }

    left * right * up * down
}
