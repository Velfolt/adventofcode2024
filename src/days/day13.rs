use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::map_res,
    multi::many0,
    sequence::tuple,
    IResult,
};

use crate::{aoc_iteratorutils::AdventOfCodeIteratorUtils, read_lines, read_to_separated_string};

use super::{AocDay, Day13};

impl AocDay for Day13 {
    fn part1() {
        let input = read_to_separated_string("inputs/day13.txt", "");
        let (_, config) = config(&input).unwrap();

        let total: i64 = config
            .iter()
            .map(|(a, b, prize)| {
                let a_inverse_coeff = 1_f64 / (a.0 * b.1 - b.0 * a.1) as f64;

                let mut ax = a_inverse_coeff * (b.1 * prize.0 + (-b.0) * prize.1) as f64;
                let mut bx = a_inverse_coeff * ((-a.1) * prize.0 + a.0 * prize.1) as f64;

                if (ax - (ax as u32) as f64).abs() > 1.0 - 0.002 {
                    ax = ax.ceil();
                }

                if (bx - (bx as u32) as f64).abs() > 1.0 - 0.002 {
                    bx = bx.ceil();
                }

                (ax, bx)
            })
            .println()
            .filter(|(a, b)| {
                ((*a - (*a as u32) as f64).abs() < 0.002
                    || (*a - (*a as u32) as f64).abs() > 1. - 0.002)
                    && ((*b - (*b as u32) as f64).abs() < 0.002
                        || (*b - (*b as u32) as f64).abs() > 1. - 0.002)
            })
            .println()
            .map(|(a, b)| a as i64 * 3 + b as i64)
            .sum();

        println!("fewest tokens {}", total);
    }

    fn part2() {
        let input = read_to_separated_string("inputs/day13.txt", "");
        let (_, config) = config(&input).unwrap();

        let total: i64 = config
            .iter()
            .map(|(a, b, prize)| (a, b, (prize.0 + 10000000000000, prize.1 + 10000000000000)))
            .map(|(a, b, prize)| {
                let a_inverse_coeff = 1_f64 / (a.0 * b.1 - b.0 * a.1) as f64;

                let mut ax = a_inverse_coeff * (b.1 * prize.0 + (-b.0) * prize.1) as f64;
                let mut bx = a_inverse_coeff * ((-a.1) * prize.0 + a.0 * prize.1) as f64;

                if (ax - (ax as u64) as f64).abs() > 1.0 - 0.0001 {
                    ax = ax.ceil();
                }

                if (bx - (bx as u64) as f64).abs() > 1.0 - 0.0001 {
                    bx = bx.ceil();
                }

                (ax, bx)
            })
            .filter(|(a, b)| {
                ((*a - (*a as u64) as f64).abs() < 0.0001
                    || (*a - (*a as u64) as f64).abs() > 1. - 0.0001)
                    && ((*b - (*b as u64) as f64).abs() < 0.0001
                        || (*b - (*b as u64) as f64).abs() > 1. - 0.0001)
            })
            .filter(|(a, b)| *a > 0. && *b > 0.)
            .map(|(a, b)| a as i64 * 3 + b as i64)
            .sum();

        println!("fewest tokens {}", total);
    }
}

fn button(input: &str) -> IResult<&str, (i64, i64)> {
    let (input, _) = tuple((tag("Button "), anychar, tag(": ")))(input)?;
    let (input, (_, x, _, y)) = tuple((
        tag("X+"),
        map_res(digit1, |s: &str| s.parse()),
        tag(", Y+"),
        map_res(digit1, |s: &str| s.parse()),
    ))(input)?;

    Ok((input, (x, y)))
}

fn prize(input: &str) -> IResult<&str, (i64, i64)> {
    let (input, _) = tag("Prize: ")(input)?;
    let (input, (_, x, _, y)) = tuple((
        tag("X="),
        map_res(digit1, |s: &str| s.parse()),
        tag(", Y="),
        map_res(digit1, |s: &str| s.parse()),
    ))(input)?;

    Ok((input, (x, y)))
}

fn config(input: &str) -> IResult<&str, Vec<((i64, i64), (i64, i64), (i64, i64))>> {
    many0(tuple((button, button, prize)))(input)
}
