use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    read_into_chars,
    utils::{IndexToPos, Point, PosToIndex, PosWithinBounds, PrintGrid},
};

use super::{AocDay, Day8};

impl AocDay for Day8 {
    fn part1() {
        let (width, grid) = read_into_chars("inputs/day8.txt");

        let mut antinodes = HashSet::new();

        grid.clone()
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != '.')
            .map(|(index, c)| (c, index.to_pos(width)))
            .into_group_map()
            .iter()
            .flat_map(|(_, b)| b.iter().combinations(2).collect_vec())
            .for_each(|a| {
                let [&a, &b] = &a[..] else { todo!() };

                let vec = Point(a) - Point(b);
                for pos in [Point(a) + Point(vec), Point(b) - Point(vec)] {
                    if pos.within_bounds(width) {
                        antinodes.insert(pos);
                    }
                }
            });

        println!("unique antinodes: {}", antinodes.len());
    }

    fn part2() {
        let (width, grid) = read_into_chars("inputs/day8.txt");

        let mut antinodes = HashSet::new();

        grid.clone()
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != '.')
            .map(|(index, c)| (c, index.to_pos(width)))
            .into_group_map()
            .iter()
            .flat_map(|(_, b)| b.iter().combinations(2).collect_vec())
            .for_each(|a| {
                let [&a, &b] = &a[..] else { todo!() };

                antinodes.insert(a);

                let vec = Point(a) - Point(b);
                let mut pos = Point(a) + Point(vec);

                while pos.within_bounds(width) {
                    antinodes.insert(pos);
                    pos = Point(pos) + Point(vec);
                }

                let mut neg_pos = Point(a) - Point(vec);

                while neg_pos.within_bounds(width) {
                    antinodes.insert(neg_pos);
                    neg_pos = Point(neg_pos) - Point(vec);
                }
            });

        println!("unique antinodes: {}", antinodes.len());
    }
}
