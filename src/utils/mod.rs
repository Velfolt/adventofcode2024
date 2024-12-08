use std::ops::{Add, Mul, Sub};

pub trait PrintGrid {
    fn print_grid(self, width: usize);
}

impl PrintGrid for Vec<char> {
    fn print_grid(self, width: usize) {
        for chunk in self.chunks(width) {
            let str: String = chunk.iter().collect();
            println!("{}", str);
        }
        println!("");
    }
}

pub trait PosWithinBounds {
    fn within_bounds(self, width: usize) -> bool;
}

impl PosWithinBounds for (i64, i64) {
    fn within_bounds(self, width: usize) -> bool {
        self.0 >= 0 && self.0 < width as i64 && self.1 >= 0 && self.1 < width as i64
    }
}

impl PosToIndex for (i64, i64) {
    fn to_index(self, width: usize) -> usize {
        (self.0 + self.1 * width as i64) as usize
    }
}

pub trait PosToIndex {
    fn to_index(self, width: usize) -> usize;
}

impl IndexToPos for usize {
    fn to_pos(self, width: usize) -> (i64, i64) {
        (self as i64 % width as i64, self as i64 / width as i64)
    }
}

pub trait IndexToPos {
    fn to_pos(self, width: usize) -> (i64, i64);
}

pub trait Distance {
    fn distance(self) -> usize;
}

impl Distance for ((i64, i64), (i64, i64)) {
    fn distance(self) -> usize {
        let (a, b) = self;
        ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize
    }
}

pub struct Point(pub (i64, i64));

impl Sub for Point {
    type Output = (i64, i64);

    fn sub(self, rhs: Self) -> Self::Output {
        (self.0 .0 - rhs.0 .0, self.0 .1 - rhs.0 .1)
    }
}

impl Add for Point {
    type Output = (i64, i64);

    fn add(self, rhs: Self) -> Self::Output {
        (self.0 .0 + rhs.0 .0, self.0 .1 + rhs.0 .1)
    }
}

impl Mul<i64> for Point {
    type Output = (i64, i64);

    fn mul(self, rhs: i64) -> Self::Output {
        (self.0 .0 * rhs, self.0 .1 * rhs)
    }
}
