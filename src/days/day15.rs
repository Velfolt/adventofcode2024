use itertools::Itertools;

use crate::{
    read_lines, read_to_separated_string,
    utils::{IndexToPos, Point, PosToIndex, PosWithinBounds, PrintGrid},
};

use super::{AocDay, Day15};

impl AocDay for Day15 {
    fn part1() {
        let binding = read_to_separated_string("inputs/day15.txt", "--");
        let (grid, directions) = binding.split("----").collect_tuple().unwrap();

        let mut width = 0;

        let mut grid = grid
            .split("--")
            .map(|s| {
                width = s.len();
                return s.to_string();
            })
            .join("")
            .chars()
            .collect_vec();

        let start_index = grid.iter().find_position(|c| **c == '@').unwrap().0;

        let mut pos = start_index.to_pos(width);

        let directions = directions.split("--").join("");

        for dir in directions.chars() {
            let dir = match dir {
                '^' => (0, -1),
                'v' => (0, 1),
                '<' => (-1, 0),
                '>' => (1, 0),
                _ => panic!(),
            };

            let next_pos = Point(pos) + Point(dir);

            match grid[next_pos.to_index(width)] {
                '#' => {}
                'O' => {
                    let mut can_move = true;
                    let mut next = next_pos;

                    let mut positions = vec![];

                    while grid[next.to_index(width)] == 'O' {
                        positions.push(next);
                        next = Point(next) + Point(dir);

                        if !next.within_bounds(width) {
                            break;
                        }

                        if grid[next.to_index(width)] == '#' {
                            can_move = false;
                            break;
                        }
                    }

                    if can_move {
                        for pos in positions {
                            let next = Point(pos) + Point(dir);
                            grid[next.to_index(width)] = 'O';
                        }

                        grid[pos.to_index(width)] = '.';
                        grid[next_pos.to_index(width)] = '@';

                        pos = next_pos
                    }
                }
                _ => {
                    grid[pos.to_index(width)] = '.';
                    grid[next_pos.to_index(width)] = '@';

                    pos = next_pos
                }
            }
        }

        let sum: i64 = grid
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == 'O')
            .map(|(i, _)| {
                let pos = i.to_pos(width);
                100 * pos.1 + pos.0
            })
            .sum();

        println!("sum of boxes gps coordinates: {}", sum);
    }

    fn part2() {
        let binding = read_to_separated_string("inputs/day15.txt", "--");
        let (grid, directions) = binding.split("----").collect_tuple().unwrap();

        let mut width = 0;

        let mut grid = grid
            .split("--")
            .map(|s| {
                width = s.len() * 2;
                return s.to_string();
            })
            .join("")
            .chars()
            .flat_map(|c| match c {
                'O' => ['[', ']'],
                '@' => ['@', '.'],
                _ => [c, c],
            })
            .collect_vec();

        let start_index = grid.iter().find_position(|c| **c == '@').unwrap().0;

        let mut pos = start_index.to_pos(width);

        let directions = directions.split("--").join("");

        for dir in directions.chars() {
            let dir = match dir {
                '^' => (0, -1),
                'v' => (0, 1),
                '<' => (-1, 0),
                '>' => (1, 0),
                _ => panic!(),
            };

            let next_pos = Point(pos) + Point(dir);

            match grid[next_pos.to_index(width)] {
                '#' => {}
                '[' | ']' => {
                    let mut can_move = true;

                    let mut positions = vec![];

                    let mut check = vec![next_pos];

                    while let Some(next) = check.pop() {
                        if !next.within_bounds(width) {
                            continue;
                        }

                        if grid[next.to_index(width)] == ']' {
                            positions.push((next.0 - 1, next.1));

                            match dir {
                                (0, -1) | (0, 1) => {
                                    check.push(Point(next) + Point(dir));
                                    check.push(Point((next.0 - 1, next.1)) + Point(dir));
                                }
                                (-1, 0) => check.push(Point(next) + Point(Point(dir) * 2)),
                                (1, 0) => check.push(Point(next) + Point(dir)),
                                _ => panic!(),
                            }
                        } else if grid[next.to_index(width)] == '[' {
                            positions.push(next);

                            match dir {
                                (0, -1) | (0, 1) => {
                                    check.push(Point(next) + Point(dir));
                                    check.push(Point((next.0 + 1, next.1)) + Point(dir));
                                }
                                (-1, 0) => check.push(Point(next) + Point(dir)),
                                (1, 0) => check.push(Point(next) + Point(Point(dir) * 2)),
                                _ => panic!(),
                            }
                        } else if grid[next.to_index(width)] == '#' {
                            can_move = false;
                        }
                    }

                    if can_move {
                        for pos in positions.iter().unique().cloned() {
                            if grid[pos.to_index(width)] == '[' {
                                grid[pos.to_index(width)] = '.';
                            }

                            if grid[pos.to_index(width) + 1] == ']' {
                                grid[pos.to_index(width) + 1] = '.';
                            }
                        }

                        for pos in positions.iter().unique().cloned() {
                            let next = Point(pos) + Point(dir);

                            grid[next.to_index(width)] = '[';
                            grid[next.to_index(width) + 1] = ']';
                        }

                        grid[pos.to_index(width)] = '.';
                        grid[next_pos.to_index(width)] = '@';

                        pos = next_pos
                    }
                }
                _ => {
                    grid[pos.to_index(width)] = '.';
                    grid[next_pos.to_index(width)] = '@';

                    pos = next_pos
                }
            }
        }

        let sum: i64 = grid
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == '[')
            .map(|(i, _)| {
                let pos = i.to_pos(width);
                100 * pos.1 + pos.0
            })
            .sum();

        println!("sum of boxes gps coordinates: {}", sum);
    }
}
