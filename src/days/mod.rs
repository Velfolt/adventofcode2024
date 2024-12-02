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

pub struct Day1;
pub struct Day2;
