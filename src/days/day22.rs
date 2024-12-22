use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

use itertools::Itertools;
use nom::sequence;

use crate::{
    aoc_iteratorutils::AdventOfCodeIteratorUtils, read_lines_into_ints, read_lines_into_strings,
};

use super::{AocDay, Day22};

impl AocDay for Day22 {
    fn part1() {
        let sum: i64 = read_lines_into_ints("inputs/day22.txt")
            .map(|initial| combo(initial, 2000, hash))
            .sum();

        println!("sum of 2000th secret: {sum}");
    }

    fn part2() {
        let all_sequences = read_lines_into_ints("inputs/day22.txt")
            .map(|initial| combo_sequences(initial, 2000, hash))
            .collect_vec();

        let mut intersections = HashSet::new();

        for sequence in all_sequences.iter() {
            let a: HashSet<&(i64, i64, i64, i64)> = HashSet::from_iter(sequence.keys());

            if intersections.is_empty() {
                intersections.extend(a);
            } else {
                let intersect =
                    intersections
                        .intersection(&a)
                        .cloned()
                        .collect::<HashSet<&(i64, i64, i64, i64)>>();

                if !intersect.is_empty() {
                    intersections.extend(intersect);
                }
            }
        }

        let (best_sequence, most_bananas) = intersections
            .iter()
            .map(|sequence| {
                (
                    sequence,
                    all_sequences
                        .iter()
                        .filter(|a| a.contains_key(sequence))
                        .map(|a| a[sequence])
                        .sum::<i64>(),
                )
            })
            .max_by_key(|x| x.1)
            .unwrap();

        println!("most bananas: {most_bananas} with the sequence {best_sequence:?}");
    }
}

fn hash(secret: i64) -> i64 {
    let mut new_secret = ((secret * 64) ^ secret) % 16777216;
    new_secret = ((new_secret / 32) ^ new_secret) % 16777216;
    new_secret = ((new_secret * 2048) ^ new_secret) % 16777216;

    new_secret
}

fn combo(initial: i64, n: usize, f: impl Fn(i64) -> i64) -> i64 {
    let mut value = initial;

    for _ in 0..n {
        value = f(value);
    }

    value
}

fn combo_sequences(
    initial: i64,
    n: usize,
    f: impl Fn(i64) -> i64,
) -> HashMap<(i64, i64, i64, i64), i64> {
    let mut value = initial;

    let mut prices = vec![];

    let mut last = initial % 10;

    for _ in 0..n {
        value = f(value);

        let rem = value.rem_euclid(10);
        prices.push((rem, rem - last));
        last = rem;
    }

    // create map that contains the first price for each sequence - the rest won't be needed.
    let mut sequences = HashMap::new();

    prices
        .iter()
        .cloned()
        .tuple_windows()
        .map(|(a, b, c, d)| (d.0, (a.1, b.1, c.1, d.1)))
        .for_each(|(price, sequence)| {
            if !sequences.contains_key(&sequence) {
                sequences.insert(sequence, price);
            }
        });

    sequences
}
