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

pub struct Day1;
pub struct Day2;
pub struct Day3;
pub struct Day4;
pub struct Day5;
