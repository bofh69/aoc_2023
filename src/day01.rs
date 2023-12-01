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
                .nth(0)
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

fn first_spelled_digit(hay: &str) -> Option<(char, usize, usize)> {
    let mut found = None;
    for (needle, c) in [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ] {
        if let Some(pos) = hay.find(needle) {
            if found == None {
                found = Some((c, pos, needle.len()))
            } else if let Some((_, old_pos, _)) = found {
                if old_pos > pos {
                    found = Some((c, pos, needle.len()))
                }
            }
        }
    }
    found
}

#[aoc(day1, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|line| {
            print!("{} becomes ", line);
            let mut line = line.to_string();
            loop {
                if let Some(found) = first_spelled_digit(&line) {
                    line.replace_range(found.1..found.1 + found.2, &found.0.to_string())
                } else {
                    break;
                }
            }
            print!("{}", line);
            let first_digit = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .nth(0)
                .expect("No first digit");
            let last_digit = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .last()
                .expect("No last digit");
            println!(" == {}{}", first_digit, last_digit);
            (first_digit * 10 + last_digit) as SolutionType
        })
        .sum::<SolutionType>()
}
