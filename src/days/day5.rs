use std::cmp::Ordering;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

use crate::read_to_separated_string;

use super::{AocDay, Day5};

fn rule(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(
        map_res(digit1, |s: &str| s.parse()),
        char('|'),
        map_res(digit1, |s: &str| s.parse()),
    )(input)
}

fn pages(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list0(char(','), map_res(digit1, |s: &str| s.parse()))(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<(usize, usize)>, Vec<Vec<usize>>)> {
    let (input, rules) = separated_list0(tag("--"), rule)(input)?;
    let (input, _) = tag("----")(input)?;
    let (input, pages) = separated_list0(tag("--"), pages)(input)?;

    Ok((input, (rules, pages)))
}

fn order_by_rules_and_filter(
    input: String,
    predicate: impl Fn(&(&mut Vec<usize>, Vec<usize>)) -> bool,
) -> usize {
    let (_, (rules, pages)) = parse(&input).unwrap();
    let sum: usize = pages
        .clone()
        .iter_mut()
        .map(|pages| {
            pages.sort_by(|a, b| {
                let rule = rules.iter().find(|rule| rule.0 == *a && rule.1 == *b);

                if let Some(_) = rule {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });
            pages
        })
        .zip(pages)
        .filter(predicate)
        .map(|(a, _)| {
            let middle = a.len() / 2;
            a[middle]
        })
        .sum();
    sum
}

impl AocDay for Day5 {
    fn part1() {
        let input = read_to_separated_string("inputs/day5.txt", "--");
        let sum = order_by_rules_and_filter(input, |(a, b)| *a == b);

        println!("{}", sum);
    }

    fn part2() {
        let input = read_to_separated_string("inputs/day5.txt", "--");
        let sum = order_by_rules_and_filter(input, |(a, b)| *a != b);

        println!("{}", sum);
    }
}
