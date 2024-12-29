use std::collections::HashSet;

use itertools::Itertools;

use crate::{aoc_iteratorutils::AdventOfCodeIteratorUtils, read_into_string_iterator};

use super::{AocDay, Day23};

impl AocDay for Day23 {
    fn part1() {
        let lines = read_into_string_iterator("inputs/day23.txt").collect_vec();

        let connections = lines
            .iter()
            .map(|x| x.split("-").collect_tuple::<(_, _)>().unwrap())
            .collect_vec();

        let mut sets = HashSet::new();

        for (a, b) in &connections {
            for (c, d) in &connections {
                for (e, f) in &connections {
                    if (a == c && b == e && d == f)
                        || (a == c && b == f && d == e)
                        || (a == d && b == f && c == e)
                        || (a == d && b == e && c == f)
                    {
                        let set = HashSet::from([a, b, c, d, e, f]);
                        sets.insert(set.iter().sorted().cloned().collect_vec());
                    }
                }
            }
        }

        let sets = sets
            .iter()
            .filter(|x| x.iter().any(|computer| computer.starts_with("t")))
            .collect_vec();

        println!(
            "Contains at least one computer that starts with t: {}",
            sets.len()
        );
    }

    fn part2() {
        let lines = read_into_string_iterator("inputs/day23.txt").collect_vec();

        let connections = lines
            .iter()
            .map(|x| x.split("-").collect_tuple::<(_, _)>().unwrap())
            .collect_vec();

        let computers = connections.iter().flat_map(|x| [x.0, x.1]).collect();

        let clique = bron_kerbosch(HashSet::new(), computers, HashSet::new(), &|vertex, r| {
            r.iter()
                .flat_map(|v2| {
                    connections
                        .iter()
                        .filter(|(a, b)| (*a == vertex) || (*b == vertex))
                        .flat_map(|x| [x.0, x.1])
                })
                .filter(|a| *a != vertex)
                .collect()
        });

        if let Some(r) = clique {
            println!("clique: {}", r.iter().sorted().join(","));
        }
    }
}

fn bron_kerbosch<'a>(
    r: HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
    n: &impl Fn(&'a str, HashSet<&'a str>) -> HashSet<&'a str>,
) -> Option<HashSet<&'a str>> {
    let mut max = None;

    if p.is_empty() && x.is_empty() {
        max = Some(r.clone());
    }

    for v in &p.clone() {
        let neighbours = n(v, &r | &HashSet::from([*v]));

        if let Some(new_r) = bron_kerbosch(
            &r | &HashSet::from([*v]),
            &p & &neighbours,
            &x & &neighbours,
            n,
        ) {
            if let Some(old_max) = &mut max {
                if new_r.len() > old_max.len() {
                    max = Some(new_r);
                }
            } else {
                max = Some(new_r);
            }
        }

        p.remove(v);
        x = &x | &HashSet::from([*v]);
    }

    max
}
