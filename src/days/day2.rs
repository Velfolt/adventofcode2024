use crate::read_lines;

use super::{AocDay, Day2};

impl Day2 {
    fn parse() -> impl Iterator<Item = Vec<i64>> {
        read_lines("inputs/day2.txt")
            .map(|line| line.unwrap())
            .map(|line| {
                line.split(" ")
                    .map(str::parse::<i64>)
                    .map(Result::unwrap)
                    .collect::<Vec<_>>()
            })
    }

    fn level_ok<'a>(levels: impl Iterator<Item = (&'a i64, &'a i64)> + Clone) -> bool {
        let all_increasing = levels.clone().all(|(a, b)| a > b);
        let all_decreasing = levels.clone().all(|(a, b)| a < b);
        let adjacent = levels.clone().all(|(a, b)| match (a - b).abs() {
            1..=3 => true,
            _ => false,
        });

        (all_decreasing || all_increasing) && adjacent
    }
}

impl AocDay for Day2 {
    fn part1() {
        let count = Self::parse()
            .filter(|levels| {
                let levels = levels.iter().zip(levels.iter().skip(1));
                Self::level_ok(levels)
            })
            .count();

        println!("{}", count)
    }

    fn part2() {
        let count = Self::parse()
            .filter(|levels| {
                if !Self::level_ok(levels.iter().zip(levels.iter().skip(1))) {
                    for x in 0..levels.len() {
                        let new_levels = levels
                            .iter()
                            .clone()
                            .enumerate()
                            .filter(|(i, _)| *i != x)
                            .map(|(_, a)| a);

                        if Self::level_ok(new_levels.clone().zip(new_levels.clone().skip(1))) {
                            return true;
                        }
                    }

                    false
                } else {
                    true
                }
            })
            .count();

        println!("{}", count)
    }
}
