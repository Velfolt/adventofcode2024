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
