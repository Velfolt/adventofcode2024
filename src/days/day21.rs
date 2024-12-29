use std::{collections::HashMap, iter};

use itertools::Itertools;
use nom::{
    bytes::complete::take_while,
    character::{complete::digit1, is_digit},
    combinator::{map, map_res},
    IResult, ParseTo,
};
use rayon::iter::{
    IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelBridge, ParallelIterator,
};

use crate::{
    aoc_iteratorutils::AdventOfCodeIteratorUtils,
    read_lines,
    utils::{Directions, Distance, IndexToPos, Point},
};

use super::{day16::astar, AocDay, Day21};

impl AocDay for Day21 {
    fn part1() {
        let codes = read_lines("inputs/day21.txt").map(|x| x.unwrap());

        let sum: usize = codes
            .map(|code| (atoi(code.as_bytes()).unwrap().1, numeric_keypad(&code)))
            .map(|(code, codes)| {
                (
                    code,
                    codes
                        .iter()
                        .map(|code| directional_keypad(&directional_keypad(code)))
                        .min_by(|a, b| a.len().cmp(&b.len()))
                        .unwrap(),
                )
            })
            .map(|(code, shortest_seq)| (code, shortest_seq.len()))
            .map(|(numeric, shortest_seq)| numeric as usize * shortest_seq)
            .sum();

        println!("sum of complexities: {sum}");
    }

    fn part2() {
        let codes = read_lines("inputs/day21.txt").map(|x| x.unwrap());

        let mut cache = HashMap::new();

        let sum: usize = codes
            .map(|code| (atoi(code.as_bytes()).unwrap().1, numeric_keypad(&code)))
            .map(|(code, codes)| {
                let robot_code = codes
                    .iter()
                    .map(|code| directional_keypad_cached(code, 25, &mut cache))
                    .min()
                    .unwrap();

                (code, robot_code)
            })
            .map(|(code, shortest_seq)| (code, shortest_seq))
            .map(|(numeric, shortest_seq)| numeric as usize * shortest_seq)
            .sum();

        println!("sum of complexities: {sum}");
    }
}

fn atoi(input: &[u8]) -> IResult<&[u8], usize> {
    map(take_while(is_digit), |s: &[u8]| s.parse_to().unwrap())(input)
}

fn numeric_keypad_position(key: char) -> (i64, i64) {
    match key {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => panic!(),
    }
}

fn numeric_keypad(code: &str) -> Vec<String> {
    let mut stack = vec![(0, 'A', vec![], (2, 3))];

    let data = code.chars().collect_vec();

    let mut out = vec![];

    while let Some((c_i, current, possible, last_pos)) = stack.pop() {
        if c_i > data.len() - 1 {
            out.push(possible);
            continue;
        }

        let c = data[c_i];
        let current_pos = last_pos;
        let c_pos = numeric_keypad_position(c);

        let max_distance = (current_pos, c_pos).distance();

        let binding = current_pos.directions();
        let next_positions = binding
            .iter()
            .cloned()
            .filter(|pos| *pos != (0, 3))
            .filter(|pos| (*pos, c_pos).distance() <= max_distance);

        for next_pos in next_positions {
            let dir = Point(next_pos) - Point(current_pos);
            let direction_char = match dir {
                (1, 0) => '>',
                (-1, 0) => '<',
                (0, 1) => 'v',
                (0, -1) => '^',
                _ => panic!(),
            };

            let mut possible = possible.clone();
            possible.push(direction_char);

            if next_pos == c_pos {
                possible.push('A');
                stack.push((c_i + 1, c, possible, next_pos));
            } else {
                stack.push((c_i, current, possible, next_pos));
            }
        }
    }

    out.iter().map(|v| v.iter().join("")).collect_vec()
}

fn directional_keypad_position(key: char) -> (i64, i64) {
    match key {
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!(),
    }
}

fn directional_keypad(code: &str) -> String {
    let mut output = vec![];

    let mut current = 'A';
    for c in code.chars() {
        let current_pos = directional_keypad_position(current);
        let c_pos = directional_keypad_position(c);

        let (mut x, mut y) = Point(c_pos) - Point(current_pos);

        if y < 0 {
            while x > 0 {
                output.push('>');
                x -= 1;
            }

            while x < 0 {
                output.push('<');
                x += 1;
            }

            while y > 0 {
                output.push('v');
                y -= 1;
            }

            while y < 0 {
                output.push('^');
                y += 1;
            }
        } else {
            while y > 0 {
                output.push('v');
                y -= 1;
            }

            while y < 0 {
                output.push('^');
                y += 1;
            }

            while x > 0 {
                output.push('>');
                x -= 1;
            }

            while x < 0 {
                output.push('<');
                x += 1;
            }
        }

        output.push('A');
        current = c;
    }

    output.into_iter().join("")
}

fn directional_keypad_cached(
    code: &str,
    n: usize,
    cache: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    let mut result = 0;

    for (start, end) in iter::once('A').chain(code.chars()).tuple_windows() {
        result += directional_keypad_mapping(start, end, n, cache);
    }

    result
}

fn directional_keypad_mapping(
    start: char,
    end: char,
    n: usize,
    cache: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    let mut result = 0;

    if let Some(length) = cache.get(&(start, end, n)) {
        result += length;
    } else {
        let out = match (start, end) {
            _ if start == end => vec!['A'],
            ('A', '^') => vec!['<', 'A'],
            ('A', '<') => vec!['v', '<', '<', 'A'],
            ('A', 'v') => vec!['<', 'v', 'A'],
            ('A', '>') => vec!['v', 'A'],

            ('^', '<') => vec!['v', '<', 'A'],
            ('^', 'v') => vec!['v', 'A'],
            ('^', '>') => vec!['v', '>', 'A'],
            ('^', 'A') => vec!['>', 'A'],

            ('<', '^') => vec!['>', '^', 'A'],
            ('<', 'v') => vec!['>', 'A'],
            ('<', '>') => vec!['>', '>', 'A'],
            ('<', 'A') => vec!['>', '>', '^', 'A'],

            ('v', '^') => vec!['^', 'A'],
            ('v', '<') => vec!['<', 'A'],
            ('v', '>') => vec!['>', 'A'],
            ('v', 'A') => vec!['^', '>', 'A'],

            ('>', '^') => vec!['<', '^', 'A'],
            ('>', '<') => vec!['<', '<', 'A'],
            ('>', 'v') => vec!['<', 'A'],
            ('>', 'A') => vec!['^', 'A'],

            _ => panic!(),
        };

        result += out.len();

        if n == 1 {
            return result;
        }

        let mut g = 0;
        if out.len() == 1 {
            g += directional_keypad_mapping(out[0], out[0], n - 1, cache);
        } else {
            for (a, b) in iter::once('A').chain(out).tuple_windows() {
                g += directional_keypad_mapping(a, b, n - 1, cache);
            }
        }
        cache.insert((start, end, n), g);

        return g;
    }

    result
}
