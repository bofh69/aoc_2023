// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use hashbrown::HashSet;

type Point = (i16, i16, i16);
type InputType = (Point, Point);
type SolutionType = usize;

fn make_point(s: &str) -> Point {
    let v: Vec<_> = s.split(',').map(|n| n.parse().expect("number")).collect();
    (v[0], v[1], v[2])
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    // 1,0,1~1,2,1
    input
        .lines()
        .map(|line| {
            let line = line.split_once('~').expect("~");
            (make_point(line.0), make_point(line.1))
        })
        .collect()
}

fn would_hit(mut a_min: Point, mut a_max: Point, b_min: Point, b_max: Point) -> bool {
    a_min.2 -= 1;
    a_max.2 -= 1;

    a_min.0 <= b_max.0
        && a_max.0 >= b_min.0
        && a_min.1 <= b_max.1
        && a_max.1 >= b_min.1
        && a_min.2 <= b_max.2
        && a_max.2 >= b_min.2
}

fn settle_bricks(data: &mut Vec<(Point, Point, bool)>) {
    data.sort_by(|a, b| a.2.cmp(&b.2));
    let mut first = 0;
    let mut last = data.len();
    while first < last {
        let mut any_moved = false;
        for i in first..last {
            if data[i].2 {
                // Its grounded
                continue;
            }
            let mut can_fall = true;
            while can_fall {
                if data[i].0 .2 == 1 {
                    // Can't fall below ground
                    data[i].2 = true;
                    if i == first {
                        first += 1;
                    }
                    break;
                }
                for j in 0..data.len() {
                    if i != j && would_hit(data[i].0, data[i].1, data[j].0, data[j].1) {
                        if data[j].2 {
                            data[i].2 = true;
                            if i == first {
                                first += 1;
                            }
                        }
                        can_fall = false;
                        break;
                    }
                }
                if can_fall {
                    data[i].0 .2 -= 1;
                    data[i].1 .2 -= 1;
                    any_moved = true;
                }
            }
            if !data[i].2 {
                last = i + 1;
            }
        }
        if !any_moved {
            break;
        }
    }
}

#[aoc(day22, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    // First point <= second point
    let mut data: Vec<_> = data.iter().map(|(p1, p2)| (*p1, *p2, p1.2 == 1)).collect();

    settle_bricks(&mut data);

    let mut supports = vec![];
    for _ in 0..data.len() {
        supports.push(vec![]);
    }
    for i in 0..data.len() {
        for j in 0..data.len() {
            if i == j {
                continue;
            }
            if !would_hit(data[j].0, data[j].1, data[i].0, data[i].1) {
                continue;
            }
            supports[i].push(j);
        }
    }
    let mut count = 0;
    for i in 0..data.len() {
        if supports[i].is_empty() {
            // println!("{} doesn't support any at all", (b'A' + i as u8) as char);
            count += 1;
            continue;
        }
        let mut is_needed = false;
        for j in &supports[i] {
            let mut has_other_support = false;
            for (k, support) in supports.iter().enumerate() {
                if k == i || k == *j {
                    continue;
                }
                if support.contains(j) {
                    // Another supports j.
                    has_other_support = true;
                    break;
                }
            }
            if !has_other_support {
                is_needed = true;
                break;
            }
        }
        if !is_needed {
            // println!( "{} can be removed, its dependees have other supports", (b'A' + i as u8) as char);
            count += 1;
        }
    }

    count
}

fn fallcount(supports: &[Vec<usize>], supported_by: &[Vec<usize>], i: usize) -> SolutionType {
    let mut removed = HashSet::new();

    let mut to_remove = Vec::from([i]);
    while let Some(i) = to_remove.pop() {
        if removed.insert(i) {
            for &j in &supports[i] {
                let mut can_fall = true;
                for &k in &supported_by[j] {
                    if !removed.contains(&k) {
                        can_fall = false;
                        break;
                    }
                }
                if can_fall {
                    to_remove.push(j);
                }
            }
        }
    }

    removed.len()
}

#[aoc(day22, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut data: Vec<_> = data.iter().map(|(p1, p2)| (*p1, *p2, p1.2 == 1)).collect();

    settle_bricks(&mut data);

    let mut supports = vec![];
    let mut supported_by = vec![];
    for _ in 0..data.len() {
        supports.push(vec![]);
        supported_by.push(vec![]);
    }
    for i in 0..data.len() {
        for j in 0..data.len() {
            if i == j {
                continue;
            }
            if !would_hit(data[j].0, data[j].1, data[i].0, data[i].1) {
                continue;
            }
            supports[i].push(j);
            supported_by[j].push(i);
        }
    }

    let mut count = 0;
    for i in 0..data.len() {
        let res = fallcount(&supports, &supported_by, i) - 1;
        // println!("{} {}", i, res);
        count += res;
    }
    count
}
