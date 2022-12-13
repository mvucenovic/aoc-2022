use std::{result, str::Chars};

use anyhow::Context;
use itertools::Itertools;

fn inputs() -> anyhow::Result<Vec<(Packet, Packet)>> {
    let input_string =
        std::fs::read_to_string("inputs/13_input.txt").context("Error while reading input")?;

    parse(input_string)
}

fn parse(input: String) -> anyhow::Result<Vec<(Packet, Packet)>> {
    Ok(input
        .split("\n\n")
        .map(|pair_lines| {
            let (left, right) = pair_lines.split_once('\n').unwrap();
            (left.into(), right.into())
        })
        .collect_vec())
}

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

impl From<&str> for Packet {
    fn from(line: &str) -> Self {
        let mut chars = line.chars();
        let first = chars.next();
        if first != Some('[') {
            panic!("unexpected input line {}", line);
        }
        parse_list(&mut chars)
    }
}

fn parse_list(chars: &mut Chars<'_>) -> Packet {
    let mut list = vec![];
    while let Some(ch) = chars.next() {
        match ch {
            ',' => continue,
            '0'..='9' => {
                let mut digits = String::from(ch);
                digits.extend(chars.peeking_take_while(|c| c.is_digit(10)));
                list.push(Packet::Int(digits.parse().unwrap()))
            }
            '[' => list.push(parse_list(chars)),
            ']' => break,
            _ => panic!("Bad input - read {}", ch),
        }
    }
    Packet::List(list)
}

fn in_order(left: &Packet, right: &Packet) -> std::cmp::Ordering {
    match (left, right) {
        (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
        (Packet::List(_), Packet::Int(r)) => in_order(left, &Packet::List(vec![Packet::Int(*r)])),
        (Packet::Int(l), Packet::List(_)) => in_order(&Packet::List(vec![Packet::Int(*l)]), right),
        (Packet::List(l), Packet::List(r)) => {
            let min_len = std::cmp::min(l.len(), r.len());
            let mut result = l.len().cmp(&r.len());
            for ix in 0..min_len {
                match in_order(&l[ix], &r[ix]) {
                    std::cmp::Ordering::Equal => continue,
                    order => {
                        result = order;
                        break;
                    }
                }
            }
            result
        }
    }
}

pub fn part_01() -> anyhow::Result<usize> {
    Ok(inputs()?
        .into_iter()
        .enumerate()
        .filter(|(_, (left, right))| {
            let ord = in_order(left, right);
            return ord == std::cmp::Ordering::Less;
        })
        .map(|(ix, _)| ix + 1)
        .sum())
}

pub fn part_02() -> anyhow::Result<usize> {
    let mut inputs = inputs()?;
    inputs.push(("[[2]]".into(), "[[6]]".into()));

    let mut result = vec![];
    for (l, r) in inputs.into_iter() {
        result.push(l);
        result.push(r);
    }

    result.sort_by(|l, r| in_order(l, r));

    Ok(
        (result.iter().position(|e| *e == "[[2]]".into()).unwrap() + 1)
            * (result.iter().position(|e| *e == "[[6]]".into()).unwrap() + 1),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_order() {
        let left = "[1,1,3,1,1]".into();
        let right = "[1,1,5,1,1]".into();

        assert_eq!(in_order(&left, &right), std::cmp::Ordering::Less);

        let left = "[[1],[2,3,4]]".into();
        let right = "[[1],4]".into();

        assert_eq!(in_order(&left, &right), std::cmp::Ordering::Less);

        let left = "[[1],[2,3,4]]".into();
        let right = "[[1],4]".into();

        assert_eq!(in_order(&left, &right), std::cmp::Ordering::Less);

        // [9] vs [[8,7,6]]
        let left = "[9]".into();
        let right = "[[8,7,6]]".into();

        assert_eq!(in_order(&left, &right), std::cmp::Ordering::Greater);

        // [[4,4],4,4] vs [[4,4],4,4,4]
        let left = "[[4,4],4,4]".into();
        let right = "[[4,4],4,4,4]".into();

        assert_eq!(in_order(&left, &right), std::cmp::Ordering::Less);

        // [[4,4],4,4,4] vs [[4,4],4,4]
        let left = "[[4,4],4,4,4]".into();
        let right = "[[4,4],4,4]".into();

        assert_eq!(in_order(&left, &right), std::cmp::Ordering::Greater);

        // [7,7,7,7] vs [7,7,7]
        let left = "[7,7,7,7]".into();
        let right = "[7,7,7]".into();

        assert_eq!(in_order(&left, &right), std::cmp::Ordering::Greater);

        // [] vs [3]
        let left = "[]".into();
        let right = "[3]".into();

        assert_eq!(in_order(&left, &right), std::cmp::Ordering::Less);

        // [[[]]] vs [[]]
        let left = "[[[]]]".into();
        let right = "[[]]".into();

        assert_eq!(in_order(&left, &right), std::cmp::Ordering::Greater);

        // [1,[2,[3,[4,[5,6,7]]]],8,9] vs [1,[2,[3,[4,[5,6,0]]]],8,9]
        let left = "[1,[2,[3,[4,[5,6,7]]]],8,9]".into();
        let right = "[1,[2,[3,[4,[5,6,0]]]],8,9]".into();

        assert_eq!(in_order(&left, &right), std::cmp::Ordering::Greater);
    }
}
