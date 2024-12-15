use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::{
    aoc_iteratorutils::AdventOfCodeIteratorUtils,
    read_into_chars,
    utils::{Directions, Distance, IndexToPos, Point, PosToIndex, PosWithinBounds},
};

use super::{AocDay, Day12};

impl AocDay for Day12 {
    fn part1() {
        let (width, input) = read_into_chars("inputs/day12.txt");

        let mut buf: HashMap<(char, (i64, i64)), Vec<(i64, i64)>> = HashMap::new();

        for i in 0..input.len() {
            if buf.values().flat_map(|x| x).contains(&i.to_pos(width)) {
                continue;
            }

            let mut current = vec![i.to_pos(width)];
            let c = input[i];

            let mut output = HashSet::new();

            while let Some(pos) = current.pop() {
                output.insert(pos);

                for next_pos in pos.directions() {
                    if next_pos.within_bounds(width)
                        && input[next_pos.to_index(width)] == c
                        && !output.contains(&next_pos)
                    {
                        current.push(next_pos)
                    }
                }
            }

            buf.insert((c, i.to_pos(width)), output.iter().cloned().collect_vec());
        }

        let price: usize = buf
            .values()
            .map(|plots| {
                let a = plots.len();
                let b = plots
                    .iter()
                    .flat_map(|pos| {
                        pos.directions()
                            .iter()
                            .filter(|x| !plots.contains(x))
                            .cloned()
                            .collect_vec()
                    })
                    .count();

                (a, b)
            })
            .map(|(a, b)| a * b)
            .sum();

        println!("total price of fencing: {}", price);
    }

    fn part2() {
        let (width, input) = read_into_chars("inputs/day12.txt");

        let mut buf: HashMap<(char, (i64, i64)), Vec<(i64, i64)>> = HashMap::new();

        for i in 0..input.len() {
            if buf.values().flat_map(|x| x).contains(&i.to_pos(width)) {
                continue;
            }

            let mut current = vec![i.to_pos(width)];
            let c = input[i];

            let mut output = HashSet::new();

            while let Some(pos) = current.pop() {
                output.insert(pos);

                for next_pos in pos.directions() {
                    if next_pos.within_bounds(width)
                        && input[next_pos.to_index(width)] == c
                        && !output.contains(&next_pos)
                    {
                        current.push(next_pos)
                    }
                }
            }

            buf.insert((c, i.to_pos(width)), output.iter().cloned().collect_vec());
        }

        let price: usize = buf
            .values()
            .map(|plots| {
                let a = plots.len();

                let sides = plots
                    .iter()
                    .filter(|pos| pos.directions().iter().any(|x| !plots.contains(x)))
                    .collect_vec();

                let perimeter = plots
                    .iter()
                    .flat_map(|pos| {
                        pos.directions()
                            .iter()
                            .filter(|x| !plots.contains(x))
                            .cloned()
                            .collect_vec()
                    })
                    .count();

                let mut count = 0;

                let mut sides = VecDeque::from(sides);

                let mut prev = None;

                while !sides.is_empty() {
                    let side = sides.pop_front().unwrap();

                    let sorted = sides
                        .iter()
                        .sorted_by(|a, b| (*side, ***a).distance().cmp(&(*side, ***b).distance()))
                        .take(1)
                        .collect_vec();

                    if let Some(next) = sorted.first() {
                        let vec = Point(***next) - Point(*side);

                        if let Some(prev) = prev {
                            println!("prev {:?} vec {:?}", prev, vec);
                            if vec != prev {
                                count += 1;
                            }
                        }

                        prev = Some(vec);
                    }
                }

                println!("");

                (a, perimeter - count)
            })
            // .println()
            .map(|(a, b)| a * b)
            .sum();

        println!("total price of fencing: {}", price);
    }
}
