use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize},
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::read_lines;

use super::{AocDay, Day14};

impl AocDay for Day14 {
    fn part1() {
        let input = read_lines("inputs/day14.txt")
            .map(|line| parse(&line.unwrap()).unwrap().1)
            .collect_vec();

        let seconds = 100;
        let (width, height) = (101, 103);

        let positions = input
            .iter()
            .map(|(p, v)| {
                (
                    (p.0 + v.0 * seconds).rem_euclid(width),
                    (p.1 + v.1 * seconds).rem_euclid(height),
                )
            })
            .filter(|(x, y)| *x != width / 2 && *y != height / 2)
            .sorted()
            .collect_vec();

        let quadrants: usize = [
            ((0, 0), (width / 2, height / 2)),
            ((width / 2 + 1, 0), (width, height / 2)),
            ((width / 2 + 1, height / 2 + 1), (width, height)),
            ((0, height / 2 + 1), (width / 2, height)),
        ]
        .iter()
        .map(|(top_left, bottom_right)| {
            positions
                .iter()
                .filter(|pos| {
                    pos.0 >= top_left.0
                        && pos.1 >= top_left.1
                        && pos.0 < bottom_right.0
                        && pos.1 < bottom_right.1
                })
                .count()
        })
        .product();

        println!("safety factor after 100 seconds: {}", quadrants);
    }

    fn part2() {
        let input = read_lines("inputs/day14.txt")
            .map(|line| parse(&line.unwrap()).unwrap().1)
            .collect_vec();

        let (width, height) = (101, 103);

        for seconds in 0..10000 {
            let positions = input.iter().map(|(p, v)| {
                (
                    (p.0 + v.0 * seconds).rem_euclid(width),
                    (p.1 + v.1 * seconds).rem_euclid(height),
                )
            });

            let max_row = positions
                .clone()
                .sorted_by(|x, y| x.1.cmp(&y.1))
                .dedup_by_with_count(|x, y| x.1 == y.1)
                .max_by(|x, y| x.0.cmp(&y.0))
                .unwrap()
                .0;

            let max_column = positions
                .clone()
                .sorted()
                .dedup_by_with_count(|x, y| x.0 == y.0)
                .max_by(|x, y| x.0.cmp(&y.0))
                .unwrap()
                .0;

            if max_row > 30 && max_column > 30 {
                println!("first easter egg at {}", seconds);
                break;
            }
        }
    }
}

fn parse(input: &str) -> IResult<&str, ((i64, i64), (i64, i64))> {
    let (input, p) = preceded(
        tag("p="),
        separated_pair(
            map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
                i64::from_str_radix(s, 10)
            }),
            char(','),
            map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
                i64::from_str_radix(s, 10)
            }),
        ),
    )(input)?;

    let (input, v) = preceded(
        tag(" v="),
        separated_pair(
            map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
                i64::from_str_radix(s, 10)
            }),
            char(','),
            map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
                i64::from_str_radix(s, 10)
            }),
        ),
    )(input)?;

    Ok((input, (p, v)))
}
