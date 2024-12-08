pub trait AocDay {
    fn perform() {
        Self::part1();
        Self::part2();
    }

    fn part1();
    fn part2();
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

pub struct Day1;
pub struct Day2;
pub struct Day3;
pub struct Day4;
pub struct Day5;
pub struct Day6;
pub struct Day7;
