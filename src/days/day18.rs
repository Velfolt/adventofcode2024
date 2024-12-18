use itertools::Itertools;

use crate::{
    read_lines,
    utils::{Distance, IndexToPos, Point, PosToIndex, PosWithinBounds, PrintGrid},
};

use super::{day16::astar, AocDay, Day18};

impl AocDay for Day18 {
    fn part1() {
        let input = read_lines("inputs/day18.txt").map(|line| {
            line.unwrap()
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect_tuple::<(i64, i64)>()
                .unwrap()
        });

        let width = 71;

        let start_index: usize = 0;
        let end_index = width * width - 1;

        let mut grid = vec!['.'; width * width];
        for pos in input.take(1024) {
            grid[pos.to_index(width)] = '#';
        }
        grid.print_grid(width);

        let (path, cost) = astar(
            (start_index.to_pos(width), (1, 0)),
            end_index.to_pos(width),
            |pos| (end_index.to_pos(width), pos.0).distance() as i32,
            |a, b| 1,
            |(pos, dir)| {
                vec![
                    (Point(pos) + Point((0, 1)), dir),
                    (Point(pos) + Point((0, -1)), dir),
                    (Point(pos) + Point((1, 0)), dir),
                    (Point(pos) + Point((-1, 0)), dir),
                ]
                .iter()
                .filter(|((pos, _))| pos.within_bounds(width) && grid[pos.to_index(width)] != '#')
                .cloned()
                .collect_vec()
            },
        )
        .unwrap();

        println!("cost {cost}");
    }

    fn part2() {
        let input = read_lines("inputs/day18.txt")
            .map(|line| {
                line.unwrap()
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .collect_tuple::<(i64, i64)>()
                    .unwrap()
            })
            .collect_vec();

        let width = 71;

        let start_index: usize = 0;
        let end_index = width * width - 1;

        let mut grid = vec!['.'; width * width];
        for pos in input.iter().take(1024) {
            grid[pos.to_index(width)] = '#';
        }

        let mut i = 0;
        let mut blocking_pos = (0, 0);

        while let Some(_) = astar(
            (start_index.to_pos(width), (1, 0)),
            end_index.to_pos(width),
            |pos| (end_index.to_pos(width), pos.0).distance() as i32,
            |a, b| 1,
            |(pos, dir)| {
                vec![
                    (Point(pos) + Point((0, 1)), dir),
                    (Point(pos) + Point((0, -1)), dir),
                    (Point(pos) + Point((1, 0)), dir),
                    (Point(pos) + Point((-1, 0)), dir),
                ]
                .iter()
                .filter(|((pos, _))| pos.within_bounds(width) && grid[pos.to_index(width)] != '#')
                .cloned()
                .collect_vec()
            },
        ) {
            let pos = input[i];
            grid[pos.to_index(width)] = '#';
            i += 1;
            blocking_pos = pos;
        }

        println!("blocking pos {:?}", blocking_pos);
    }
}
