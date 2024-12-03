use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{anychar, char},
    combinator::{map, map_res},
    multi::{many0, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::read_lines;

use super::{AocDay, Day3};

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul((i64, i64)),
    Do,
    Dont,
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, b)) = many_till(
        anychar,
        alt((
            map(
                delimited(
                    tag("mul("),
                    separated_pair(
                        map_res(take_while_m_n(1, 3, |a: char| a.is_digit(10)), |s: &str| {
                            s.parse()
                        }),
                        char(','),
                        map_res(take_while_m_n(1, 3, |a: char| a.is_digit(10)), |s: &str| {
                            s.parse()
                        }),
                    ),
                    char(')'),
                ),
                |s| Instruction::Mul(s),
            ),
            map(tag("do()"), |_| Instruction::Do),
            map(tag("don't()"), |_| Instruction::Dont),
        )),
    )(input)?;

    Ok((input, b))
}

fn parse_instruction(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(instruction)(input)
}

impl AocDay for Day3 {
    fn part1() {
        let count = read_lines("inputs/day3.txt")
            .map(|line| line.unwrap())
            .flat_map(|line| parse_instruction(&line).unwrap().1)
            .filter_map(|instr| {
                if let Instruction::Mul(a) = instr {
                    Some(a)
                } else {
                    None
                }
            })
            .map(|(a, b)| a * b)
            .sum::<i64>();

        println!("{:?}", count);
    }

    fn part2() {
        let iter = read_lines("inputs/day3.txt")
            .map(|line| line.unwrap())
            .flat_map(|line| parse_instruction(&line).unwrap().1);

        let mut sum = 0;
        let mut enabled = true;

        for instr in iter {
            match instr {
                Instruction::Mul((a, b)) => {
                    if enabled {
                        sum += a * b;
                    }
                }
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = false,
            }
        }

        println!("{}", sum);
    }
}
