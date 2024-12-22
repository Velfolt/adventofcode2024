use std::{
    fs::File,
    io::{self, BufRead},
};

use days::AocDay;
use itertools::Itertools;

mod aoc_iteratorutils;
mod days;
mod utils;

fn main() {
    // days::Day1::perform();
    // days::Day2::perform();
    // days::Day3::perform();
    // days::Day4::perform();
    // days::Day5::perform();
    // days::Day6::perform();
    // days::Day7::perform();
    // days::Day8::perform();
    // days::Day9::perform();
    // days::Day10::perform();
    // days::Day11::perform();
    // days::Day12::perform();
    // days::Day13::perform();
    // days::Day14::perform();
    // days::Day15::perform();
    // days::Day16::perform();
    // days::Day17::perform();
    // days::Day18::perform();
    // days::Day19::perform();
    // days::Day20::perform();
    days::Day21::perform();
}

pub fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

pub fn read_into_ints(filename: &str) -> Vec<u32> {
    read_lines(filename)
        .map(|line| {
            let line = line.unwrap();
            line
        })
        .join("")
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

pub fn read_into_chars(filename: &str) -> (usize, Vec<char>) {
    let mut width = 0;

    let input = read_lines(filename)
        .map(|line| {
            let line = line.unwrap();
            width = line.len();
            line
        })
        .join("")
        .chars()
        .collect_vec();

    (width, input)
}

pub fn read_to_separated_string(filename: &str, sep: &str) -> String {
    let input = read_lines(filename)
        .map(|line| {
            let line = line.unwrap();
            line
        })
        .join(sep);

    input
}

pub fn read_into_string_iterator(filename: &str) -> impl Iterator<Item = String> {
    read_lines(filename).map(|line| line.unwrap())
}
