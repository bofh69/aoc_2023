// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

type InputType = Vec<i64>;
type SolutionType = i64;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse().expect("Number"))
                .collect()
        })
        .collect()
}

fn predict_next(data: &[i64]) -> i64 {
    let mut diffs = vec![];
    let mut is_all_zero = true;
    for i in 1..data.len() {
        let diff = data[i] - data[i - 1];
        diffs.push(diff);
        if diff != 0 {
            is_all_zero = false;
        }
    }
    if is_all_zero {
        data[data.len() - 1]
    } else {
        data[data.len() - 1] + predict_next(&diffs)
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.iter().map(|line| predict_next(line)).sum()
}

#[aoc(day9, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|line| {
            let mut line = line.to_vec();
            line.reverse();
            predict_next(&line)
        })
        .sum()
}
