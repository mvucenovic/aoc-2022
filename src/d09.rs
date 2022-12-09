use anyhow::Context;
use std::collections::HashSet;

fn inputs() -> anyhow::Result<String> {
    let input_string =
        std::fs::read_to_string("inputs/09_input.txt").context("Error while reading input")?;

    Ok(input_string)
}

pub fn part_01() -> anyhow::Result<usize> {
    let inputs = inputs()?;

    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut tail_positions = HashSet::new();
    tail_positions.insert(tail);

    for line in inputs.lines() {
        let line = line.trim();
        let (ch, num) = line.split_once(' ').context("bad format")?;

        let ch = ch.chars().next().context("missing char")?;

        let vector = match ch {
            'U' => (0, -1),
            'D' => (0, 1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!("Unknown char {}", ch),
        };

        let num = num.parse::<usize>()?;

        for _ in 0..num {
            head = (head.0 + vector.0, head.1 + vector.1);
            tail = move_tail(head, tail);
            tail_positions.insert(tail);
        }
    }

    Ok(tail_positions.len())
}

fn move_tail(head: (i32, i32), mut tail: (i32, i32)) -> (i32, i32) {
    if (head.0 - tail.0).abs() >= 2 || (head.1 - tail.1).abs() >= 2 {
        (tail.0, tail.1) = (
            tail.0 + (head.0 - tail.0).signum(),
            tail.1 + (head.1 - tail.1).signum(),
        )
    }
    tail
}

pub fn part_02() -> anyhow::Result<usize> {
    let inputs = inputs()?;

    let mut rope = vec![(0, 0); 10];
    let mut tail_positions = HashSet::new();
    tail_positions.insert(rope[9]);

    for line in inputs.lines() {
        let line = line.trim();
        let (ch, num) = line.split_once(' ').context("bad format")?;

        let ch = ch.chars().next().context("missing char")?;

        let vector = match ch {
            'U' => (0, -1),
            'D' => (0, 1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!("Unknown char {}", ch),
        };

        let num = num.parse::<usize>()?;

        for _ in 0..num {
            rope[0] = (rope[0].0 + vector.0, rope[0].1 + vector.1);
            for ix in 1..10 {
                rope[ix] = move_tail(rope[ix - 1], rope[ix]);
            }
            tail_positions.insert(rope[9]);
        }
    }

    Ok(tail_positions.len())
}
