use std::collections::{BinaryHeap, HashMap, HashSet};

use anyhow::Context;
use itertools::Itertools;

fn inputs() -> anyhow::Result<Map> {
    let input_string =
        std::fs::read_to_string("inputs/12_input.txt").context("Error while reading input")?;

    parse(input_string)
}

type Map = ((usize, usize), (usize, usize), Vec<Vec<i8>>);

fn parse(input: String) -> anyhow::Result<Map> {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut map = vec![];
    for (iy, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (ix, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = (ix, iy);
                row.push(0)
            } else if ch == 'E' {
                end = (ix, iy);
                row.push('z' as i8 - 'a' as i8)
            } else {
                row.push(ch as i8 - 'a' as i8)
            }
        }
        map.push(row);
    }
    Ok((start, end, map))
}

fn neighbours(coord: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .map(|i| (i.0 + coord.0 as i64, i.1 + coord.1 as i64))
        .filter(|&(x, y)| x < width as i64 && x >= 0 && y < height as i64 && y >= 0)
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Node {
    coord: (usize, usize),
    cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn a_start(start: (usize, usize), end: (usize, usize), map: &Vec<Vec<i8>>) -> i32 {
    let height = map.len();
    let width = map[0].len();

    let mut open_set = BinaryHeap::new();
    let mut visited = HashSet::new();

    open_set.push(Node {
        coord: start,
        cost: 0,
    });
    visited.insert(start);

    while !open_set.is_empty() {
        let current = open_set.pop().unwrap();

        if current.coord == end {
            return current.cost;
        }

        let current_height = map[current.coord.1][current.coord.0];
        neighbours(current.coord, width, height)
            .into_iter()
            .filter(|&(ix, iy)| map[iy][ix] - 1 <= current_height)
            .for_each(|coord| {
                if !visited.contains(&coord) {
                    open_set.push(Node {
                        coord,
                        cost: current.cost + 1,
                    });
                    visited.insert(coord);
                }
            })
    }

    i32::MAX
}

pub fn part_01() -> anyhow::Result<i32> {
    let (start, end, map) = inputs()?;

    Ok(a_start(start, end, &map))
}

pub fn part_02() -> anyhow::Result<i32> {
    let (_, end, map) = inputs()?;
    let height = map.len();
    let width = map[0].len();

    let mut results = vec![];
    for ix in 0..width {
        for iy in 0..height {
            if map[iy][ix] == 0 {
                results.push(a_start((ix, iy), end, &map));
            }
        }
    }

    results.sort();
    Ok(results[0])
}
