use std::{
    borrow::BorrowMut,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
    i32,
};

use itertools::{GroupingMap, Itertools};

use crate::{
    aoc_iteratorutils::AdventOfCodeIteratorUtils,
    read_into_chars,
    utils::{Distance, IndexToPos, Point, PosToIndex, PrintGrid},
};

use super::{AocDay, Day16};

impl AocDay for Day16 {
    fn part1() {
        let (width, grid) = read_into_chars("inputs/day16.txt");

        let start_index = grid.iter().find_position(|x| **x == 'S').unwrap().0;
        let goal_index = grid.iter().find_position(|x| **x == 'E').unwrap().0;

        let path = astar(
            (start_index.to_pos(width), (1, 0)),
            goal_index.to_pos(width),
            |pos| (goal_index.to_pos(width), pos.0).distance() as i32 * 1000,
            |a, b| {
                let diff = Point(a.1) - Point(b.1);

                if diff.0 != 0 && diff.1 != 0 {
                    1000
                } else {
                    1
                }
            },
            |(pos, dir)| {
                match dir {
                    (1, 0) => vec![
                        (Point(pos) + Point(dir), dir),
                        (pos, (0, 1)),
                        (pos, (0, -1)),
                    ],
                    (-1, 0) => vec![
                        (Point(pos) + Point(dir), dir),
                        (pos, (0, 1)),
                        (pos, (0, -1)),
                    ],
                    (0, 1) => vec![
                        (Point(pos) + Point(dir), dir),
                        (pos, (1, 0)),
                        (pos, (-1, 0)),
                    ],
                    (0, -1) => vec![
                        (Point(pos) + Point(dir), dir),
                        (pos, (1, 0)),
                        (pos, (-1, 0)),
                    ],
                    _ => panic!(),
                }
                .iter()
                .filter(|((pos, _))| grid[pos.to_index(width)] != '#')
                .cloned()
                .collect_vec()
            },
        )
        .unwrap();

        println!("cost {}", path.1);
    }

    fn part2() {
        let (width, grid) = read_into_chars("inputs/day16.txt");

        let start_index = grid.iter().find_position(|x| **x == 'S').unwrap().0;
        let goal_index = grid.iter().find_position(|x| **x == 'E').unwrap().0;

        let (dist, prev) = dijkstra(
            grid.iter()
                .enumerate()
                .filter(|(_, c)| **c != '#')
                .flat_map(|(i, _)| {
                    [
                        (i.to_pos(width), (1, 0)),
                        (i.to_pos(width), (-1, 0)),
                        (i.to_pos(width), (0, 1)),
                        (i.to_pos(width), (0, -1)),
                    ]
                })
                .collect_vec(),
            (start_index.to_pos(width), (1, 0)),
            goal_index.to_pos(width),
            |(pos, dir)| {
                match dir {
                    (1, 0) => vec![
                        (Point(pos) + Point(dir), dir),
                        (pos, (0, 1)),
                        (pos, (0, -1)),
                    ],
                    (-1, 0) => vec![
                        (Point(pos) + Point(dir), dir),
                        (pos, (0, 1)),
                        (pos, (0, -1)),
                    ],
                    (0, 1) => vec![
                        (Point(pos) + Point(dir), dir),
                        (pos, (1, 0)),
                        (pos, (-1, 0)),
                    ],
                    (0, -1) => vec![
                        (Point(pos) + Point(dir), dir),
                        (pos, (1, 0)),
                        (pos, (-1, 0)),
                    ],
                    _ => panic!(),
                }
                .iter()
                .filter(|((pos, _))| grid[pos.to_index(width)] != '#')
                .cloned()
                .collect_vec()
            },
            |a, b| {
                let diff = Point(a.1) - Point(b.1);

                if diff.0 != 0 && diff.1 != 0 {
                    1000
                } else {
                    1
                }
            },
        );

        let paths = shortest_path(goal_index.to_pos(width), prev.clone());

        let iter = paths.iter().map(|path| {
            let cost: usize = path
                .clone()
                .iter()
                .zip(path.clone().iter().skip(1))
                .map(|(a, b)| {
                    let diff = Point(a.1) - Point(b.1);

                    if diff.0 != 0 && diff.1 != 0 {
                        1000
                    } else {
                        1
                    }
                })
                .sum();

            (cost, path)
        });

        let paths = iter.clone().min_set_by(|a, b| a.0.cmp(&b.0));
        let positions = paths
            .clone()
            .iter()
            .flat_map(|(_, paths)| paths.iter().map(|(pos, _)| pos).collect_vec())
            .unique()
            .collect_vec();

        println!("unique positions {}", positions.len());
    }
}

pub fn astar(
    start: ((i64, i64), (i64, i64)),
    goal: (i64, i64),
    h: impl Fn(((i64, i64), (i64, i64))) -> i32,
    d: impl Fn(((i64, i64), (i64, i64)), ((i64, i64), (i64, i64))) -> i32,
    neighbours: impl Fn(((i64, i64), (i64, i64))) -> Vec<((i64, i64), (i64, i64))>,
) -> Option<(Vec<((i64, i64), (i64, i64))>, i32)> {
    let mut open_set = BinaryHeap::from([Reverse((0, start))]);

    let mut came_from = HashMap::new();

    let mut gscore = HashMap::new();
    gscore.insert(start, 0);

    let mut fscore = HashMap::new();
    fscore.insert(start, h(start));

    while !open_set.is_empty() {
        if let Some(Reverse((_, current))) = open_set.pop() {
            if current.0 == goal {
                return Some((reconstruct_path(&came_from, current), gscore[&current]));
            }

            for neighbour in neighbours(current) {
                let tentative_gscore = gscore[&current] + d(current, neighbour);
                if tentative_gscore < *gscore.get(&neighbour).or(Some(&i32::MAX)).unwrap() {
                    came_from
                        .entry(neighbour)
                        .and_modify(|x| *x = current)
                        .or_insert(current);

                    gscore
                        .entry(neighbour)
                        .and_modify(|x| *x = tentative_gscore)
                        .or_insert(tentative_gscore);

                    fscore
                        .entry(neighbour)
                        .and_modify(|x| *x = tentative_gscore + h(neighbour))
                        .or_insert(tentative_gscore + h(neighbour));

                    if open_set
                        .iter()
                        .find(|x| match **x {
                            Reverse((_, x)) => x == neighbour,
                            _ => false,
                        })
                        .is_none()
                    {
                        open_set.push(std::cmp::Reverse((tentative_gscore, neighbour)));
                    }
                }
            }
        }
    }

    None
}

fn reconstruct_path(
    came_from: &HashMap<((i64, i64), (i64, i64)), ((i64, i64), (i64, i64))>,
    mut current: ((i64, i64), (i64, i64)),
) -> Vec<((i64, i64), (i64, i64))> {
    let mut total_path = vec![current];
    while came_from.contains_key(&current) {
        current = came_from[&current];
        total_path.insert(0, current);
    }
    total_path
}

fn dijkstra(
    graph: Vec<((i64, i64), (i64, i64))>,
    start: ((i64, i64), (i64, i64)),
    goal: (i64, i64),
    neighbours: impl Fn(((i64, i64), (i64, i64))) -> Vec<((i64, i64), (i64, i64))>,
    d: impl Fn(((i64, i64), (i64, i64)), ((i64, i64), (i64, i64))) -> i32,
) -> (
    HashMap<((i64, i64), (i64, i64)), i32>,
    HashMap<((i64, i64), (i64, i64)), Vec<((i64, i64), (i64, i64))>>,
) {
    let mut q = vec![];

    let mut dist = HashMap::new();
    let mut prev = HashMap::new();

    for pos in graph {
        dist.entry(pos).insert_entry(i32::MAX);

        q.push(pos);
    }

    dist.entry(start).insert_entry(0);

    while !q.is_empty() {
        if let Some((k, v)) = dist.iter().min_by_key(|(k, v)| **v) {
            if let Some((u_index, u)) = q.clone().iter().find_position(|x| *x == k) {
                q.remove(u_index);

                if u.1 == goal {
                    break;
                }

                for neighbour in neighbours(*u).iter().filter(|xx| q.contains(xx)) {
                    let alt = dist[&u] + d(*u, *neighbour);

                    if alt <= dist[neighbour] {
                        dist.entry(*neighbour).and_modify(|x| *x = alt);
                        prev.entry(*neighbour)
                            .and_modify(|x: &mut Vec<((i64, i64), (i64, i64))>| x.push(*u))
                            .or_insert(vec![*u]);
                    }
                }

                dist.remove(u);
            }
        }
    }

    (dist, prev)
}

fn shortest_path(
    target: (i64, i64),
    prev: HashMap<((i64, i64), (i64, i64)), Vec<((i64, i64), (i64, i64))>>,
) -> Vec<Vec<((i64, i64), (i64, i64))>> {
    let mut stack = vec![
        (vec![(target, (1, 0))], prev.get(&(target, (1, 0)))),
        (vec![(target, (-1, 0))], prev.get(&(target, (-1, 0)))),
        (vec![(target, (0, -1))], prev.get(&(target, (0, -1)))),
        (vec![(target, (0, 1))], prev.get(&(target, (0, 1)))),
    ];

    let mut output = vec![];

    while let Some((mut out, previous_nodes)) = stack.pop() {
        if let Some(nodes) = previous_nodes {
            for x in nodes {
                let mut out = out.clone();
                out.push(*x);

                stack.push((out, prev.get(x)));
            }
        } else {
            output.push(out);
        }
    }

    output
}
