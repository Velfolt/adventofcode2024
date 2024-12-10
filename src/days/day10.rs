use itertools::Itertools;

use crate::{
    read_into_chars,
    utils::{Directions, IndexToPos, PosToIndex, PosWithinBounds},
};

use super::{AocDay, Day10};

fn paths(initial: &Vec<(i64, i64)>, width: usize, input: &Vec<u32>) -> Vec<Vec<(i64, i64)>> {
    let mut paths = initial.iter().cloned().map(|pos| vec![pos]).collect_vec();

    let mut output = vec![];

    while let Some(path) = paths.pop() {
        if let Some(&pos) = path.last() {
            if input[pos.to_index(width)] == 9 {
                output.push(path);
                continue;
            }

            pos.directions()
                .iter()
                .filter(|x| x.within_bounds(width))
                .filter(|x| input[x.to_index(width)] == input[pos.to_index(width)] + 1)
                .for_each(|next_pos| {
                    let mut path = path.clone();
                    path.push(*next_pos);
                    paths.push(path);
                });
        }
    }

    output
}

impl AocDay for Day10 {
    fn part1() {
        let (width, input) = read_into_chars("inputs/day10.txt");

        let input = input
            .iter()
            .map(|c| c.to_digit(10).unwrap_or(11))
            .collect_vec();

        let initial_zeroes = input
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == 0)
            .map(|(i, _)| i.to_pos(width))
            .collect_vec();

        let sum: usize = paths(&initial_zeroes, width, &input)
            .iter()
            .unique_by(|x| (x.first().unwrap(), x.last().unwrap()))
            .dedup_by_with_count(|x, y| x.first().unwrap() == y.first().unwrap())
            .map(|(count, _)| count)
            .sum();

        println!("all trailheads: {}", sum);
    }

    fn part2() {
        let (width, input) = read_into_chars("inputs/day10.txt");

        let input = input
            .iter()
            .map(|c| c.to_digit(10).unwrap_or(11))
            .collect_vec();

        let initial_zeroes = input
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == 0)
            .map(|(i, _)| i.to_pos(width))
            .collect_vec();

        let sum: usize = paths(&initial_zeroes, width, &input)
            .iter()
            .dedup_by_with_count(|x, y| x.first().unwrap() == y.first().unwrap())
            .map(|(count, _)| count)
            .sum();

        println!("all trailhead ratings: {}", sum);
    }
}
