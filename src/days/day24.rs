use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1},
    combinator::{map, map_res},
    multi::{fold_many0, separated_list1},
    sequence::{separated_pair, tuple},
    IResult,
};

use crate::{aoc_iteratorutils::AdventOfCodeIteratorUtils, read_to_separated_string};

use super::{AocDay, Day24};

impl AocDay for Day24 {
    fn part1() {
        let binding = read_to_separated_string("inputs/day24.txt", "--");
        let (initial, gates) = binding.split("----").collect_tuple().unwrap();

        let wires = parse_initial(initial).unwrap().1;
        let gates = VecDeque::from(parse_gates(gates).unwrap().1);

        let wires = simulate(wires, gates);

        let z = wires.to_decimal("z");

        println!("What decimal number does it output on the wires starting with z: {z}");
    }

    fn part2() {
        let binding = read_to_separated_string("inputs/day24.txt", "--");
        let (_, gates) = binding.split("----").collect_tuple().unwrap();

        let gates = parse_gates(gates).unwrap().1;

        let mut swaps = HashSet::new();

        for (l, gate, r, out) in &gates {
            if (l.starts_with("x") && r.starts_with("y"))
                || (l.starts_with("y") && r.starts_with("x"))
            {
                if ![Gate::Xor, Gate::And].contains(&gate) {
                    swaps.insert(out.to_string());
                }

                if *gate == Gate::Xor {
                    let next = gates.next_gates(out);
                    let xor_and = next
                        .iter()
                        .all(|(_, gate, _, _)| [Gate::Xor, Gate::And].contains(&gate));

                    if (!xor_and || next.len() != 2) && *out != "z00" {
                        swaps.insert(out.to_string());
                    }

                    for (a, b, c, d) in next {
                        if b == Gate::Xor && !d.starts_with("z") {
                            swaps.insert(d.to_string());
                        }
                    }
                }

                if *gate == Gate::And {
                    let next = gates.next_gates(out);
                    let or = next
                        .iter()
                        .all(|(_, gate, _, _)| [Gate::Or].contains(&gate));

                    if (!or || next.len() != 1) && (*l != "x00" && *r != "y00") {
                        swaps.insert(out.to_string());
                    }

                    for (a, b, c, d) in next {
                        if b == Gate::And && (*l != "x00" && *r != "y00") {
                            swaps.insert(out.to_string());
                        }
                    }
                }
            }

            if out.starts_with("z") && *gate != Gate::Xor && *out != "z45" {
                swaps.insert(out.to_string());
            }
        }

        println!("{}", swaps.iter().sorted().join(","));
    }
}

fn simulate<'a>(
    mut wires: HashMap<&'a str, i64>,
    mut gates: VecDeque<(&'a str, Gate, &'a str, &'a str)>,
) -> HashMap<&'a str, i64> {
    while let Some((l, gate, r, output)) = gates.pop_front() {
        // if not all inputs exists, put back in gates
        if !wires.contains_key(l) || !wires.contains_key(r) {
            gates.push_back((l, gate, r, output));
            continue;
        }

        let l = wires[l];
        let r = wires[r];
        let out = match gate {
            Gate::And => l & r,
            Gate::Or => l | r,
            Gate::Xor => l ^ r,
        };

        wires.insert(output, out);
    }

    wires
}

fn parse_initial(input: &str) -> IResult<&str, HashMap<&str, i64>> {
    fold_many0(
        separated_list1(
            tag("--"),
            separated_pair(
                alphanumeric1,
                tag(": "),
                map_res(digit1, |s: &str| s.parse()),
            ),
        ),
        HashMap::new,
        |mut map, pairs| {
            map.extend(pairs);
            map
        },
    )(input)
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Gate {
    And,
    Or,
    Xor,
}

impl From<&str> for Gate {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            _ => unreachable!(),
        }
    }
}

fn parse_gates(input: &str) -> IResult<&str, Vec<(&str, Gate, &str, &str)>> {
    separated_list1(
        tag("--"),
        map(
            tuple((
                alphanumeric1,
                tag(" "),
                alphanumeric1,
                tag(" "),
                alphanumeric1,
                tag(" -> "),
                alphanumeric1,
            )),
            |(l, _, gate, _, r, _, out)| (l, Gate::from(gate), r, out),
        ),
    )(input)
}

trait ToDecimal {
    fn to_decimal(&self, starts_with: &str) -> i64;
}

impl ToDecimal for HashMap<&str, i64> {
    fn to_decimal(&self, starts_with: &str) -> i64 {
        self.iter()
            .filter(|(k, v)| k.starts_with(starts_with))
            .sorted()
            .rev()
            .fold(0, |acc, (_, bit)| (acc << 1) + bit)
    }
}

trait GateTravel {
    fn next_gates<'a>(&'a self, output: &str) -> Vec<(&'a str, Gate, &'a str, &'a str)>;
}

impl GateTravel for Vec<(&str, Gate, &str, &str)> {
    fn next_gates<'a>(&'a self, output: &str) -> Vec<(&'a str, Gate, &'a str, &'a str)> {
        self.iter()
            .filter(|(l, gate, r, out)| *l == output || *r == output)
            .cloned()
            .collect_vec()
    }
}

/// generate a graphviz string to show all nodes with suspected swapped outputs
///
/// idea from cryon!
fn graphviz(
    wires: &HashMap<&str, i64>,
    gates: &Vec<(&str, Gate, &str, &str)>,
    swaps: &HashSet<String>,
) {
    let edges = gates
        .iter()
        .map(|(l, gate, r, out)| format!("{{{l},{r}}} -> {out}[label={gate:?}]"))
        .sorted()
        .join(";\n");

    let filled_wires = wires
        .iter()
        .map(|(k, v)| format!("{k} [style=filled]"))
        .sorted()
        .join(";\n");

    let outputs = gates
        .iter()
        .filter(|(_, _, _, out)| out.starts_with("z"))
        .map(|(l, gate, r, out)| format!("{out} [style=filled,color=blue]"))
        .sorted()
        .join(";\n");

    let suspects = swaps
        .iter()
        .sorted()
        .map(|out| format!("{out} [style=filled,color=red]"))
        .join(";\n");

    println!("digraph G {{ {edges} {filled_wires} {outputs} {suspects} }}}}");
}
