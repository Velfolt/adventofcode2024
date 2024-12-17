use core::panic;

use itertools::Itertools;
use nom::InputTakeAtPosition;

use crate::read_to_separated_string;

use super::{AocDay, Day17};

impl AocDay for Day17 {
    fn part1() {
        let (program, (a, b, c)) = parse();
        let (output, _) = run(&program, &a, &b, &c);

        println!("output {}", output.iter().join(","));
    }

    fn part2() {
        let (program, _) = parse();
        let possibilities = find_minimum_a(&program);
        let minimum = possibilities.iter().min().unwrap();

        println!("minimum {:?}", minimum);
    }
}

fn parse() -> (Vec<i64>, (i64, i64, i64)) {
    let input = read_to_separated_string("inputs/day17.txt", "--");
    let (registers, program) = input.split("----").collect_tuple().unwrap();

    let [a, b, c] = registers
        .split("--")
        .flat_map(|register| register.split(": ").skip(1).collect_vec())
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec()[..]
    else {
        panic!()
    };

    let program = program
        .split(": ")
        .skip(1)
        .flat_map(|s| s.split(",").map(|s| s.parse::<i64>().unwrap()))
        .collect_vec();

    (program, (a, b, c))
}

fn find_minimum_a(program: &Vec<i64>) -> Vec<i64> {
    let mut possibilities = vec![0];

    for output in program.iter().rev() {
        let mut suspects = vec![];

        for a in possibilities {
            for i in 0..8 {
                let a = 8 * a + i;
                let b = a % 8;
                let b = b ^ 1;
                let c = (a as f64 / 2f64.powf(b as f64)).trunc() as i64;
                let b = b ^ 5;
                let b = b ^ c;

                if b % 8 == *output {
                    suspects.push(a);
                }
            }
        }

        possibilities = suspects.iter().filter(|x| **x > 0).cloned().collect_vec();
        if possibilities.is_empty() {
            break;
        }
    }

    possibilities
}

fn run(program: &Vec<i64>, a: &i64, b: &i64, c: &i64) -> (Vec<i64>, (i64, i64, i64)) {
    let mut output = vec![];

    let mut pc = 0;

    let (mut a, mut b, mut c) = (*a, *b, *c);

    loop {
        if pc + 1 > program.len() {
            break;
        }

        let (opcode, operand) = (program[pc], program[pc + 1]);

        match opcode {
            0 => a = a / 2_f64.powf(combo(&operand, &a, &b, &c) as f64).trunc() as i64,
            1 => b = b ^ operand,
            2 => b = combo(&operand, &a, &b, &c) % 8,
            3 => {
                if a != 0 {
                    pc = operand as usize;
                    continue;
                }
            }
            4 => b = b ^ c,
            5 => {
                output.push(combo(&operand, &a, &b, &c) % 8);
            }
            6 => b = a / 2_f64.powf(combo(&operand, &a, &b, &c) as f64).trunc() as i64,
            7 => c = a / 2_f64.powf(combo(&operand, &a, &b, &c) as f64).trunc() as i64,
            _ => panic!(),
        }

        pc += 2;
    }

    (output, (a, b, c))
}

fn combo(operand: &i64, a: &i64, b: &i64, c: &i64) -> i64 {
    match operand {
        0..4 => *operand,
        4 => *a,
        5 => *b,
        6 => *c,
        _ => unreachable!(),
    }
}
