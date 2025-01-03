use itertools::Itertools;

use crate::{aoc_iteratorutils::AdventOfCodeIteratorUtils, read_to_separated_string};

use super::{AocDay, Day25};

#[derive(Debug, Hash, Eq, PartialEq)]
enum Type {
    Lock,
    Key,
}

impl AocDay for Day25 {
    fn part1() {
        let lines = read_to_separated_string("inputs/day25.txt", "--");
        let group = lines
            .split("----")
            .map(|schematic| {
                let schematic = schematic.replace("--", "");
                if schematic[0..1] == *"." {
                    (
                        Type::Key,
                        (0..5)
                            .map(|x| {
                                for y in 0..7 {
                                    if schematic[(y * 5 + x)..(y * 5 + x + 1)] == *"#" {
                                        return (7 - y - 1) as i64;
                                    }
                                }
                                7
                            })
                            .collect_vec(),
                    )
                } else {
                    (
                        Type::Lock,
                        (0..5)
                            .map(|x| {
                                for y in 0..7 {
                                    if schematic[(y * 5 + x)..(y * 5 + x + 1)] == *"." {
                                        return (y - 1) as i64;
                                    }
                                }
                                7
                            })
                            .collect_vec(),
                    )
                }
            })
            .into_group_map();

        let locks = group[&Type::Lock].clone();
        let keys = group[&Type::Key].clone();

        let mut unique = 0;

        for lock in locks {
            for key in &keys {
                if lock.iter().zip(key).all(|(a, b)| 6 - a - b > 0) {
                    unique += 1;
                }
            }
        }

        println!("How many unique lock/key pairs fit together without overlapping in any column: {unique}");
    }

    fn part2() {}
}
