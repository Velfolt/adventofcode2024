use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::read_lines;

use super::{AocDay, Day11};

impl AocDay for Day11 {
    fn part1() {
        let input: Vec<i64> = read_lines("inputs/day11.txt")
            .flat_map(|line| {
                line.unwrap()
                    .split(" ")
                    .map(|x| x.parse().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        let mut input = VecDeque::from(input);

        for _ in 0..25 {
            let mut output = vec![];
            while let Some(stone) = input.pop_front() {
                let digits = stone.checked_ilog10().unwrap_or(0) + 1;

                match stone {
                    0 => output.push(1),
                    even_stone if digits % 2 == 0 => {
                        output.push(even_stone / 10_i64.pow(digits / 2));
                        output.push(even_stone % 10_i64.pow(digits / 2));
                    }
                    _ => output.push(stone * 2024),
                }
            }
            input = output.into();
        }

        println!("stones after 25 blinks: {}", input.len());
    }

    fn part2() {
        let input: Vec<i64> = read_lines("inputs/day11.txt")
            .flat_map(|line| {
                line.unwrap()
                    .split(" ")
                    .map(|x| x.parse().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        let counts = count_stones(&input, 75);

        println!("stones after 75 blinks: {}", counts.values().sum::<usize>());
    }
}

fn count_stones(input: &Vec<i64>, n: usize) -> HashMap<i64, usize> {
    let mut counts = input.iter().cloned().counts();

    for _ in 0..n {
        let mut current_counts = HashMap::new();

        for stone in counts.keys() {
            let digits = stone.checked_ilog10().unwrap_or(0) + 1;
            let next_stones = match stone {
                0 => vec![1],
                even_stone if digits % 2 == 0 => {
                    vec![
                        even_stone / 10_i64.pow(digits / 2),
                        even_stone % 10_i64.pow(digits / 2),
                    ]
                }
                _ => vec![stone * 2024],
            };

            for next_stone in next_stones {
                current_counts
                    .entry(next_stone)
                    .and_modify(|x| *x += counts[stone])
                    .or_insert(counts[stone]);
            }
        }

        counts = current_counts;
    }

    counts
}
