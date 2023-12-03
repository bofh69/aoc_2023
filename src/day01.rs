// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

type InputType = String;
type SolutionType = i32;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    let mut result = vec![];
    for line in input.lines() {
        result.push(line.to_string());
    }
    result
}

#[aoc(day1, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|line| {
            let first_digit = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .next()
                .expect("No first digit");
            let last_digit = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .last()
                .expect("No last digit");
            (first_digit * 10 + last_digit) as SolutionType
        })
        .sum::<SolutionType>()
}

#[aoc(day1, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|line| {
            let mut line = line.to_string();
            for (from, to) in [
                ("one", "o1e"),
                ("two", "t2o"),
                ("three", "t3e"),
                ("four", "f4r"),
                ("five", "f5e"),
                ("six", "s6x"),
                ("seven", "s7n"),
                ("eight", "e8t"),
                ("nine", "n9e"),
            ] {
                line = line.replace(from, to);
            }
            let first_digit = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .next()
                .expect("No first digit");
            let last_digit = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .last()
                .expect("No last digit");
            (first_digit * 10 + last_digit) as SolutionType
        })
        .sum::<SolutionType>()
}
