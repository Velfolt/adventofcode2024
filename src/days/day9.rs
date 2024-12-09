use crate::read_into_ints;

use super::{AocDay, Day9};

impl AocDay for Day9 {
    fn part1() {
        let input = read_into_ints("inputs/day9.txt");

        let mut disk = vec![];
        let mut id_counter: i64 = 0;

        for (i, x) in input.iter().enumerate() {
            if i % 2 == 0 {
                for _ in 0..*x {
                    disk.push(id_counter)
                }
                id_counter += 1;
            } else {
                for _ in 0..*x {
                    disk.push(-1)
                }
            }
        }

        let mut reverse_i = disk.len() - 1;

        for i in 0..disk.len() {
            let x = disk[i];
            while disk[reverse_i] == -1 {
                reverse_i -= 1;
            }
            if i >= reverse_i {
                break;
            }
            if x != -1 {
                continue;
            }

            disk[i] = disk[reverse_i];
            disk[reverse_i] = -1;
        }

        let sum: i64 = disk
            .iter()
            .enumerate()
            .filter(|(_, x)| **x != -1)
            .map(|(i, x)| i as i64 * *x)
            .sum();

        println!("filesystem checksum {}", sum);
    }

    fn part2() {
        let input = read_into_ints("inputs/day9.txt");

        let mut disk = vec![];
        let mut id_counter: i64 = 0;

        for (i, &x) in input.iter().enumerate() {
            if i % 2 == 0 {
                disk.push((id_counter, x));
                id_counter += 1;
            } else {
                disk.push((-1, x))
            }
        }

        for i in (0..disk.len()).rev() {
            let (revx, revamount) = disk[i];

            if revx == -1 {
                continue;
            }

            // find free pos
            for y in 0..i {
                let (x, amount) = disk[y];

                if x == -1 && revamount <= amount {
                    if amount - revamount > 0 {
                        disk[y] = disk[i];
                        disk.insert(y + 1, (-1, amount - revamount));
                        disk[i + 1] = (-1, revamount);
                    } else {
                        disk[y] = disk[i];
                        disk[i] = (-1, revamount);
                    }

                    break;
                }
            }
        }

        let mut index = 0;
        let mut sum = 0;

        for (x, amount) in disk {
            if x == -1 {
                index += amount;
                continue;
            }

            for _ in 0..amount {
                sum += x * index as i64;
                index += 1;
            }
        }

        println!("filesystem checksum {}", sum);
    }
}
