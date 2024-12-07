use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    read_into_chars,
    utils::{IndexToPos, PosToIndex, PosWithinBounds},
};

use super::{AocDay, Day6};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn walk(
    width: usize,
    input: &mut Vec<char>,
    start_pos: (i64, i64),
    start_direction: Direction,
) -> Option<HashSet<((i64, i64), Direction)>> {
    let mut direction = start_direction;
    let mut pos = start_pos;
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&(pos, direction)) {
            return None;
        }

        visited.insert((pos, direction));
        input[pos.to_index(width)] = 'X';

        let possible_pos = match direction {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        };

        if !possible_pos.within_bounds(width) {
            break;
        }

        if input[possible_pos.to_index(width)] == '#' {
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        } else {
            pos = possible_pos;
        }
    }

    Some(visited)
}

impl AocDay for Day6 {
    fn part1() {
        let (width, mut input) = read_into_chars("inputs/day6.txt");
        let (start_index, _) = input.iter().find_position(|c| **c == '^').unwrap();
        let visited = walk(width, &mut input, start_index.to_pos(width), Direction::Up).unwrap();

        println!(
            "unique positions {:?}",
            visited.iter().unique_by(|(pos, _)| pos).count()
        );
    }

    fn part2() {
        let (width, mut input) = read_into_chars("inputs/day6.txt");
        let new_input = input.clone();
        let (start_index, _) = input.iter().find_position(|c| **c == '^').unwrap();
        let visited = walk(width, &mut input, start_index.to_pos(width), Direction::Up).unwrap();

        let obstructions = visited
            .iter()
            .filter_map(|(pos, dir)| {
                let mut my_input = new_input.clone();
                my_input[pos.to_index(width)] = '#';
                if let None = walk(
                    width,
                    &mut my_input,
                    start_index.to_pos(width),
                    Direction::Up,
                ) {
                    Some(pos)
                } else {
                    None
                }
            })
            .unique()
            .count();

        println!("obstructions: {}", obstructions);
    }
}
