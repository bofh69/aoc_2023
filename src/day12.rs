// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use rayon::prelude::*;
use std::collections::HashMap;

type InputType = (Vec<u8>, Vec<u8>);
type SolutionType = usize;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    // .??..??...?##. 1,1,3
    input
        .lines()
        .map(|line| {
            let line = line.split_once(' ').expect("No game separator");
            let springs = Vec::from(line.0.as_bytes());
            let groups = line
                .1
                .split(',')
                .map(|n| n.parse().expect("number"))
                .collect();
            (springs, groups)
        })
        .collect()
}

type HashKey<'a> = (bool, u8, &'a [u8], u8, &'a [u8]);

fn count_arrangements<'a>(
    cache: &mut HashMap<HashKey<'a>, SolutionType>,
    is_inside_group: bool,
    current_spring: u8,
    springs: &'a [u8],
    left: u8,
    next_groups: &'a [u8],
) -> SolutionType {
    let key = (is_inside_group, current_spring, springs, left, next_groups);
    if let Some(result) = cache.get(&key) {
        return *result;
    }
    let result = count_arrangements_(
        cache,
        is_inside_group,
        current_spring,
        springs,
        left,
        next_groups,
    );
    cache.insert(key, result);

    result
}

fn count_arrangements_<'a>(
    cache: &mut HashMap<HashKey<'a>, SolutionType>,
    is_inside_group: bool,
    current_spring: u8,
    springs: &'a [u8],
    left: u8,
    next_groups: &'a [u8],
) -> SolutionType {
    if springs.is_empty() {
        return match (is_inside_group, current_spring) {
            (_, b'?') => {
                count_arrangements_(cache, is_inside_group, b'#', springs, left, next_groups)
                    + count_arrangements_(cache, is_inside_group, b'.', springs, left, next_groups)
            }
            (true, b'#') => {
                if left == 0 {
                    0
                } else if left == 1 && next_groups.is_empty() {
                    1
                } else {
                    0
                }
            }
            (true, b'.') => {
                if left == 0 && next_groups.is_empty() {
                    1
                } else {
                    0
                }
            }
            (false, b'#') => {
                if next_groups.len() == 1 && next_groups[0] == 1 {
                    1
                } else {
                    0
                }
            }
            (false, b'.') => {
                if next_groups.is_empty() {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        };
    }
    match (is_inside_group, current_spring) {
        (true, b'?') => {
            count_arrangements(cache, is_inside_group, b'#', springs, left, next_groups)
                + count_arrangements(cache, is_inside_group, b'.', springs, left, next_groups)
        }
        (false, b'?') => {
            count_arrangements_(cache, is_inside_group, b'#', springs, left, next_groups)
                + count_arrangements_(cache, is_inside_group, b'.', springs, left, next_groups)
        }
        (true, b'#') => {
            if left == 0 {
                0
            } else {
                count_arrangements(
                    cache,
                    is_inside_group,
                    springs[0],
                    &springs[1..],
                    left - 1,
                    next_groups,
                )
            }
        }
        (true, b'.') => {
            if left != 0 {
                0
            } else {
                count_arrangements(cache, false, springs[0], &springs[1..], 0, next_groups)
            }
        }
        (false, b'#') => {
            if next_groups.is_empty() {
                0
            } else {
                count_arrangements(
                    cache,
                    true,
                    springs[0],
                    &springs[1..],
                    next_groups[0] - 1,
                    &next_groups[1..],
                )
            }
        }
        (false, b'.') => {
            count_arrangements_(cache, false, springs[0], &springs[1..], 0, next_groups)
        }
        _ => unreachable!(),
    }
}

#[aoc(day12, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.par_iter()
        .map(|(springs, groups)| {
            let mut cache = HashMap::new();
            count_arrangements(&mut cache, false, springs[0], &springs[1..], 0, groups)
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.par_iter()
        .map(|(spring, group)| {
            let mut spring2 = vec![];
            let mut groups2 = vec![];
            spring2.extend_from_slice(spring);
            groups2.extend_from_slice(group);
            for _ in 0..4 {
                spring2.push(b'?');
                spring2.extend_from_slice(spring);
                groups2.extend_from_slice(group);
            }
            (spring2, groups2)
        })
        .map(|(springs, groups)| {
            let mut cache = HashMap::new();
            count_arrangements(&mut cache, false, springs[0], &springs[1..], 0, &groups)
        })
        .sum()
}
