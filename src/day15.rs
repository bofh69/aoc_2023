// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

// use std::collections::HashSet;

type InputType = String;
type SolutionType = usize;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    // rn=1,cm-,qp=3
    input.split(',').map(|s| s.to_string()).collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|s| {
            s.as_bytes().iter().fold(0u8, |acc, c| {
                acc.overflowing_add(*c).0.overflowing_mul(17).0
            })
        })
        .map(|n| SolutionType::from(n))
        .sum()
}

#[aoc(day15, part2)]
pub fn solve_part2(_data: &[InputType]) -> SolutionType {
    let mut boxes : Vec<Vec<(u8, u8)>> = Vec::new();
    for _i in 0..256 {
        boxes.push(Vec::new());
    }
    /*
    let labels: Vec<_> = data
        .iter()
        .map(|s| {
            s.as_bytes().iter()
                .take_while(|&&c| c != b'-' && c != b'=')
                .fold(0u8, |acc, &c| {
                acc.overflowing_add(c).0.overflowing_mul(17).0
            })
        })
        .map(|n| usize::from(n))
        .collect();

    let focals: Vec<Option<u8>> = data
        .iter()
        .map(|s| {
            if let Some((_, n)) = s.split_once('=') {
                Some(n.parse().expect("number"))
            } else {
                None
            }
        })
        .collect();

    for (i, &_label) in labels.iter().enumerate() {
        if let Some(_n) = focals[i] {
            // Update in place or add at end
        } else {
            // Remove
        }
    }
    println!("{:?} {:?}", labels, focals);
    */
    0
}
