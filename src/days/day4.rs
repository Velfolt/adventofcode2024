use itertools::Itertools;

use crate::{read_into_chars, read_lines};

use super::{AocDay, Day4};

impl Day4 {
    fn search_word<'a>(
        width: &'a usize,
        directions: &'a Vec<(i32, i32)>,
        search: &'a str,
        input: &'a Vec<char>,
    ) -> impl Iterator<Item = ((i64, i64), &'a (i32, i32), String)> {
        (0..width * width).flat_map(move |i| {
            let pos = (i % width, i / width);

            directions
                .iter()
                .filter_map(|dir| {
                    let word: String = (0..(search.len()))
                        .filter_map(|len| {
                            let pos_with_dir = (
                                pos.0 as i64 + dir.0 as i64 * len as i64,
                                pos.1 as i64 + dir.1 as i64 * len as i64,
                            );

                            let index = pos_with_dir.0 + pos_with_dir.1 * *width as i64;
                            if pos_with_dir.0 < 0
                                || pos_with_dir.0 > *width as i64 - 1
                                || pos_with_dir.1 < 0
                                || pos_with_dir.1 > *width as i64 - 1
                            {
                                return None;
                            }

                            Some(input[index as usize])
                        })
                        .collect();

                    let middle_pos = (pos.0 as i64 + dir.0 as i64, pos.1 as i64 + dir.1 as i64);

                    match word == search {
                        true => Some((middle_pos, dir, word)),
                        false => None,
                    }
                })
                .collect_vec()
        })
    }
}

impl AocDay for Day4 {
    fn part1() {
        let (width, input) = read_into_chars("inputs/day4.txt");

        let all_directions = vec![
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];

        let words = Self::search_word(&width, &all_directions, "XMAS", &input);

        println!("count {}", words.count())
    }

    fn part2() {
        let (width, input) = read_into_chars("inputs/day4.txt");

        let cross_directions = vec![(1, 1), (-1, 1), (-1, -1), (1, -1)];

        let words = Self::search_word(&width, &cross_directions, "MAS", &input)
            .sorted_by(|(pos, _, _), (pos2, _, _)| Ord::cmp(pos, pos2))
            .dedup_by_with_count(|(pos, _, _), (pos2, _, _)| pos == pos2)
            .filter(|(count, _)| *count == 2);

        println!("count {}", words.count())
    }
}
