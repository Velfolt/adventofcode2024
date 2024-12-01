use crate::read_lines;

use super::{AocDay, Day1};

impl AocDay for Day1 {
    fn part1() {
        let (mut l, mut r): (Vec<_>, Vec<_>) = read_lines("inputs/day1.txt")
            .map(|line| line.unwrap())
            .map(|line| {
                let split = line
                    .split("   ")
                    .map(str::parse::<i32>)
                    .map(Result::unwrap)
                    .collect::<Vec<_>>();
                (split[0], split[1])
            })
            .unzip();

        l.sort();
        r.sort();

        let sum: i32 = l.iter().cloned().zip(r).map(|(a, b)| (a - b).abs()).sum();

        println!("{:?}", sum)
    }

    fn part2() {
        let (l, r): (Vec<_>, Vec<_>) = read_lines("inputs/day1.txt")
            .map(|line| line.unwrap())
            .map(|line| {
                let split = line
                    .split("   ")
                    .map(str::parse::<i32>)
                    .map(Result::unwrap)
                    .collect::<Vec<_>>();
                (split[0], split[1])
            })
            .unzip();

        let sum: i64 = l
            .iter()
            .map(|a| (*a as i64) * r.iter().filter(|b| *b == a).count() as i64)
            .sum();

        println!("{:?}", sum)
    }
}
