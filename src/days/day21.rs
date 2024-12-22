use itertools::Itertools;
use nom::{
    bytes::complete::take_while,
    character::{complete::digit1, is_digit},
    combinator::{map, map_res},
    IResult, ParseTo,
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
            .map(|code| (atoi(code.as_bytes()).unwrap().1, numeric_keypad2(&code)))
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

        let sum: usize = codes
            .println()
            .map(|code| (atoi(code.as_bytes()).unwrap().1, numeric_keypad2(&code)))
            .map(|(code, codes)| {
                let robot_code = codes
                    .iter()
                    .map(|code| directional_keypad(code))
                    .map(|code| directional_keypad(&code))
                    .min_by(|a, b| a.len().cmp(&b.len()))
                    .unwrap();

                (code, robot_code)
            })
            .println()
            // .map(|(code, numeric)| (code, directional_keypad2(&numeric)))
            // .println()
            // .map(|(code, directional)| (code, directional_keypad2(&directional)))
            // .println()
            .map(|(code, shortest_seq)| (code, shortest_seq.len()))
            .println()
            .map(|(numeric, shortest_seq)| numeric as usize * shortest_seq)
            .println()
            .sum();

        println!("sum of complexities: {sum}");
    }
}

fn atoi(input: &[u8]) -> IResult<&[u8], i64> {
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

fn numeric_keypad(code: &str) -> String {
    let mut output = vec![];

    let mut current = 'A';
    for c in code.chars() {
        let mut current_pos = numeric_keypad_position(current);
        let c_pos = numeric_keypad_position(c);

        loop {
            if c == current {
                break;
            }

            let next_pos = current_pos
                .directions()
                .iter()
                .cloned()
                .filter(|pos| *pos != (0, 3))
                .min_by(|a, b| (*a, c_pos).distance().cmp(&(*b, c_pos).distance()))
                .unwrap();

            println!("next pos {next_pos:?}");
            let dir = Point(next_pos) - Point(current_pos);
            let direction_char = match dir {
                (1, 0) => '>',
                (-1, 0) => '<',
                (0, 1) => 'v',
                (0, -1) => '^',
                _ => panic!(),
            };

            output.push(direction_char);

            if next_pos == c_pos {
                break;
            }

            current_pos = next_pos.clone();
        }

        output.push('A');
        current = c;
    }

    output.into_iter().join("")
}

fn numeric_keypad2(code: &str) -> Vec<String> {
    let mut stack = vec![(0, 'A', vec![], (2, 3))];

    let data = code.chars().collect_vec();

    let mut out = vec![];

    while let Some((c_i, current, possible, last_pos)) = stack.pop() {
        // println!("{c_i} {current} {possible:?} {last_pos:?}");

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

fn directional_keypad2(code: &str) -> String {
    let mut output = vec![];

    let mut current = 'A';
    for c in code.chars() {
        let current_pos = directional_keypad_position(current);
        let c_pos = directional_keypad_position(c);

        let out = match (current, c) {
            _ if current == c => vec![],
            ('A', '^') => vec!['<'],
            ('A', '<') => vec!['v', '<', '<'],
            ('A', 'v') => vec!['<', 'v'],
            ('A', '>') => vec!['v'],

            ('^', '<') => vec!['v', '<'],
            ('^', 'v') => vec!['v'],
            ('^', '>') => vec!['v', '>'],
            ('^', 'A') => vec!['>'],

            ('<', '^') => vec!['>', '^'],
            ('<', 'v') => vec!['>'],
            ('<', '>') => vec!['>', '>'],
            ('<', 'A') => vec!['>', '>', '^'],

            ('v', '^') => vec!['^'],
            ('v', '<') => vec!['<'],
            ('v', '>') => vec!['>'],
            ('v', 'A') => vec!['>', '^'],

            ('>', '^') => vec!['^', '<'],
            ('>', '<') => vec!['<', '<'],
            ('>', 'v') => vec!['>'],
            ('>', 'A') => vec!['^'],

            _ => panic!(),
        };

        output.extend(out);
        output.push('A');

        current = c;
    }

    output.into_iter().join("")
}

fn directional_keypad3(code: &str) -> String {
    let mut output = vec![];

    let mut current = 'A';
    for c in code.chars() {
        let current_pos = directional_keypad_position(current);
        let c_pos = directional_keypad_position(c);

        let (path, cost) = astar(
            (current_pos, (1, 0)),
            c_pos,
            |pos| (c_pos, pos.0).distance() as i32,
            |a, b| {
                let dir = Point(b.0) - Point(a.0);

                match dir {
                    (1, 0) => 1,
                    (-1, 0) => 1,
                    (0, 1) => 2,
                    (0, -1) => 2,
                    _ => panic!(),
                }
            },
            |(pos, dir)| {
                vec![
                    (Point(pos) + Point((0, 1)), dir),
                    (Point(pos) + Point((0, -1)), dir),
                    (Point(pos) + Point((1, 0)), dir),
                    (Point(pos) + Point((-1, 0)), dir),
                ]
                .iter()
                .filter(|(pos, _)| *pos != (0, 0))
                .cloned()
                .collect_vec()
            },
        )
        .unwrap();

        for ((pos, _), (pos2, _)) in path.iter().zip(path.iter().skip(1)) {
            let dir = Point(*pos2) - Point(*pos);
            let direction_char = match dir {
                (1, 0) => '>',
                (-1, 0) => '<',
                (0, 1) => 'v',
                (0, -1) => '^',
                _ => panic!(),
            };

            output.push(direction_char);
        }

        output.push('A');

        current = c;
    }

    output.into_iter().join("")
}
