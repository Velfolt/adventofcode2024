use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{
    read_into_chars,
    utils::{Directions, IndexToPos, Point, PosToIndex, PosWithinBounds},
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
            .iter()
            .map(|((_, _), plots)| {
                let a = plots.len();

                let outer_corners = plots
                    .clone()
                    .iter()
                    .map(|pos| {
                        let corners = [
                            (Point(*pos) + Point((0, -1)), Point(*pos) + Point((-1, 0))),
                            (Point(*pos) + Point((0, 1)), Point(*pos) + Point((-1, 0))),
                            (Point(*pos) + Point((0, 1)), Point(*pos) + Point((1, 0))),
                            (Point(*pos) + Point((0, -1)), Point(*pos) + Point((1, 0))),
                        ];

                        corners
                    })
                    .flat_map(|corners| {
                        corners
                            .iter()
                            .map(|(a, b)| !plots.contains(&a) && !plots.contains(&b))
                            .collect_vec()
                    })
                    .collect_vec();

                let inner_corners = plots
                    .clone()
                    .iter()
                    .map(|pos| {
                        let corners = [
                            (
                                Point(*pos) + Point((0, -1)),
                                Point(*pos) + Point((-1, 0)),
                                Point(*pos) + Point((-1, -1)),
                            ),
                            (
                                Point(*pos) + Point((0, 1)),
                                Point(*pos) + Point((-1, 0)),
                                Point(*pos) + Point((-1, 1)),
                            ),
                            (
                                Point(*pos) + Point((0, 1)),
                                Point(*pos) + Point((1, 0)),
                                Point(*pos) + Point((1, 1)),
                            ),
                            (
                                Point(*pos) + Point((0, -1)),
                                Point(*pos) + Point((1, 0)),
                                Point(*pos) + Point((1, -1)),
                            ),
                        ];

                        corners
                    })
                    .flat_map(|corners| {
                        corners
                            .iter()
                            .map(|(a, b, c)| {
                                plots.contains(&a) && plots.contains(&b) && !plots.contains(&c)
                            })
                            .collect_vec()
                    })
                    .collect_vec();

                let count = outer_corners.iter().filter(|x| **x == true).count()
                    + inner_corners.iter().filter(|x| **x == true).count();

                (a, count)
            })
            .map(|(a, b)| a * b)
            .sum();

        println!("total price of fencing: {}", price);
    }
}
