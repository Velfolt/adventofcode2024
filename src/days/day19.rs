use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
};

use itertools::Itertools;

use crate::{aoc_iteratorutils::AdventOfCodeIteratorUtils, read_to_separated_string};

use super::{AocDay, Day19};

impl AocDay for Day19 {
    fn part1() {
        let input = read_to_separated_string("inputs/day19.txt", "--");
        let (towels, designs) = input.split("----").collect_tuple().unwrap();

        let towels = towels.split(", ").collect_vec();
        let designs = designs.split("--").collect_vec();

        let sum = designs
            .iter()
            .filter(|design| possible_design(design, &towels))
            .count();

        println!("Designs possible: {sum}");
    }

    fn part2() {
        let input = read_to_separated_string("inputs/day19.txt", "--");
        let (towels, designs) = input.split("----").collect_tuple().unwrap();

        let towels = towels.split(", ").collect_vec();
        let designs = designs.split("--").collect_vec();

        let mut memo = HashMap::new();

        let sum: usize = designs
            .iter()
            .map(|design| possible_designs(design, &towels, &mut memo))
            .sum();

        println!("Sum of total designs possible: {sum}");
    }
}

fn possible_design(design: &str, towels: &Vec<&str>) -> bool {
    let mut stack = BinaryHeap::from([Reverse((design.len(), design))]);

    let mut visited = vec![];

    while let Some(Reverse((len, design))) = stack.pop() {
        for towel in towels.iter().filter(|towel| design.starts_with(**towel)) {
            let rest = &design[towel.len()..];

            if rest.is_empty() {
                return true;
            }

            if !visited.contains(&rest) {
                visited.push(&rest);
                stack.push(Reverse((rest.len(), rest)));
            }
        }
    }

    false
}

fn possible_designs<'a>(
    design: &'a str,
    towels: &Vec<&str>,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(count) = memo.get(design) {
        return *count;
    }

    let mut count = 0;

    for towel in towels.iter().filter(|towel| design.starts_with(**towel)) {
        let rest = &design[towel.len()..];

        if let Some(k) = memo.get(rest) {
            count += k;
        } else {
            let k = possible_designs(rest, towels, memo);
            memo.insert(rest, k);

            count += k;
        }
    }

    count
}
