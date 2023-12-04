// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashSet;

type InputType = (HashSet<u16>, HashSet<u16>);
type SolutionType = usize;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    input
        .lines()
        .map(|line| line.split_once(": ").expect("No Card separator").1)
        .map(|card| {
            // 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            let cards = card.split_once(" | ").expect("No cards separator");
            (
                cards
                    .0
                    .split_ascii_whitespace()
                    .map(|num| num.parse().expect("Number"))
                    .collect(),
                cards
                    .1
                    .split_ascii_whitespace()
                    .map(|num| num.parse().expect("Number"))
                    .collect(),
            )
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|(win, my)| win.intersection(my).count())
        .filter(|num| *num > 0)
        .map(|num| 1 << (num - 1))
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut n_copies = Vec::with_capacity(data.len());
    n_copies.resize_with(data.len(), || 1);

    data.iter()
        .enumerate()
        .map(|(n, (win, my))| (n + 1, win.intersection(my).count()))
        .filter(|(_, wins)| *wins > 0)
        .for_each(|(n, wins)| {
            for i in n..n + wins {
                if i < data.len() {
                    n_copies[i] += n_copies[n - 1];
                }
            }
        });

    n_copies.iter().sum()
}
