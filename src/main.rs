use std::{
    fs::File,
    io::{self, BufRead},
};

// use aoc_iteratorutils::AdventOfCodeIteratorUtils;
use days::AocDay;

mod aoc_iteratorutils;
mod days;

fn main() {
    days::Day1::perform();
    days::Day2::perform();
    days::Day3::perform();
}

pub fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
