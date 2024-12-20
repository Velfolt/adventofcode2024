use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    days::day16::astar,
    read_into_chars,
    utils::{Directions, Distance, IndexToPos, Point, PosToIndex, PosWithinBounds},
};

use super::{AocDay, Day20};

impl AocDay for Day20 {
    fn part1() {
        let (width, grid) = read_into_chars("inputs/day20.txt");

        let start_index = grid.iter().find_position(|x| **x == 'S').unwrap().0;
        let goal_index = grid.iter().find_position(|x| **x == 'E').unwrap().0;

        let (path, cost) = astar(
            (start_index.to_pos(width), (1, 0)),
            goal_index.to_pos(width),
            |pos| (goal_index.to_pos(width), pos.0).distance() as i32,
            |a, b| 1,
            |(pos, dir)| {
                vec![
                    (Point(pos) + Point((0, 1)), dir),
                    (Point(pos) + Point((0, -1)), dir),
                    (Point(pos) + Point((1, 0)), dir),
                    (Point(pos) + Point((-1, 0)), dir),
                ]
                .iter()
                .filter(|(pos, _)| grid[pos.to_index(width)] != '#')
                .cloned()
                .collect_vec()
            },
        )
        .unwrap();

        let mut cheats = HashMap::new();

        for (i, (pos, _)) in path.iter().enumerate() {
            let directions = (0, 0).directions();

            let cheat_positions = directions
                .iter()
                .map(|dir| Point(*pos) + Point(Point(*dir) * 2))
                .filter(|pos| pos.within_bounds(width));

            for cheat_pos in cheat_positions {
                if let '.' | 'E' = grid[cheat_pos.to_index(width)] {
                    if let Some((cheat_i, _)) = path
                        .clone()
                        .iter()
                        .enumerate()
                        .find(|pos| pos.1 .0 == cheat_pos)
                    {
                        let saving = cheat_i as i64 - i as i64 - 2;
                        if saving >= 100 {
                            cheats.entry(saving).and_modify(|x| *x += 1).or_insert(1);
                        }
                    }
                }
            }
        }

        let sum: usize = cheats
            .iter()
            .filter_map(|(k, v)| if *k >= 100 { Some(v) } else { None })
            .sum();

        println!("cheats that would save at least 100 ps: {sum}");
    }

    fn part2() {
        let (width, grid) = read_into_chars("inputs/day20.txt");

        let start_index = grid.iter().find_position(|x| **x == 'S').unwrap().0;
        let goal_index = grid.iter().find_position(|x| **x == 'E').unwrap().0;

        let (path, cost) = astar(
            (start_index.to_pos(width), (1, 0)),
            goal_index.to_pos(width),
            |pos| (goal_index.to_pos(width), pos.0).distance() as i32,
            |a, b| 1,
            |(pos, dir)| {
                vec![
                    (Point(pos) + Point((0, 1)), dir),
                    (Point(pos) + Point((0, -1)), dir),
                    (Point(pos) + Point((1, 0)), dir),
                    (Point(pos) + Point((-1, 0)), dir),
                ]
                .iter()
                .filter(|(pos, _)| grid[pos.to_index(width)] != '#')
                .cloned()
                .collect_vec()
            },
        )
        .unwrap();

        let mut cheats = HashMap::new();

        for (i, (pos, _)) in path.iter().enumerate() {
            for goal_index in (i)..path.len() {
                let cheat_cost = (path[goal_index].0, *pos).distance();

                if cheat_cost > 20 {
                    continue;
                }

                let saving: i64 = goal_index as i64 - i as i64 - cheat_cost as i64;
                if saving >= 100 {
                    cheats.entry(saving).and_modify(|x| *x += 1).or_insert(1);
                }
            }
        }

        let sum: usize = cheats.values().sum();

        println!("cheats that would save at least 100 ps: {sum}");
    }
}
