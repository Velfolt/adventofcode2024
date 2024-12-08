use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::many1,
    sequence::preceded, IResult,
};

use crate::read_into_string_iterator;

use super::{AocDay, Day7};

fn parse(input: &str) -> IResult<&str, (i128, Vec<i128>)> {
    let (input, result) = map_res(digit1, |s: &str| s.parse())(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, values) = many1(preceded(tag(" "), map_res(digit1, |s: &str| s.parse())))(input)?;

    Ok((input, (result, values)))
}

fn determine_operator(result: i128, rest: &[i128], with_concat: bool) -> bool {
    let last = rest[0];
    let rest = &rest[1..];

    if rest.is_empty() {
        return last == result;
    }

    if result % last == 0 && determine_operator(result / last, rest, with_concat) {
        return true;
    }

    if with_concat {
        let digit_multiplier = 10_i128.pow(last.checked_ilog10().unwrap_or(0) + 1);

        if (result / digit_multiplier) * digit_multiplier + last == result
            && determine_operator(result / digit_multiplier, rest, with_concat)
        {
            return true;
        }
    }

    determine_operator(result - last, rest, with_concat)
}

impl AocDay for Day7 {
    fn part1() {
        let sum: i128 = read_into_string_iterator("inputs/day7.txt")
            .map(|s| parse(&s).unwrap().1)
            .filter_map(|(result, values)| {
                determine_operator(
                    result,
                    &values.iter().rev().cloned().collect_vec()[..],
                    false,
                )
                .then(|| result)
            })
            .sum();

        println!("total calibration result {:?}", sum);
    }

    fn part2() {
        let sum: i128 = read_into_string_iterator("inputs/day7.txt")
            .map(|s| parse(&s).unwrap().1)
            .filter_map(|(result, values)| {
                determine_operator(
                    result,
                    &values.iter().rev().cloned().collect_vec()[..],
                    true,
                )
                .then(|| result)
            })
            .sum();

        println!("total calibration result (with concat) {:?}", sum);
    }
}
