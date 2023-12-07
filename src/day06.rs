// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

type InputType = (u64, u64);
type SolutionType = u64;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    // Time:    7    15   30
    let mut input = input.lines().map(|line| {
        line.split_once(": ")
            .expect("No header separator")
            .1
            .split_ascii_whitespace()
            .map(|num| num.parse().expect("Number"))
    });
    let times = input.next().expect("Time line");
    let distances = input.next().expect("Distance line");
    times.zip(distances).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|&(time, dist)| {
            let delta = time * time - 4 * dist;
            let p1 = (time as f64 + f64::sqrt(delta as f64)) / 2.0;
            let p2 = (time as f64 - f64::sqrt(delta as f64)) / 2.0;
            let mut min = f64::ceil(p1.min(p2)) as u64;
            let mut max = f64::floor(p1.max(p2)) as u64;
            if min * (time - min) <= dist {
                min += 1
            }
            if max * (time - max) <= dist {
                max -= 1
            }
            1 + max - min
        })
        .product()
}

#[aoc(day6, part2)]
pub fn solve_part2(_data: &[InputType]) -> SolutionType {
    // Solved with part 1 and modified input file...
    0
}
