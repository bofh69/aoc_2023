// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

type InputType = String;
type SolutionType = usize;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    // rn=1,cm-,qp=3
    input.split(',').map(|s| s.to_string()).collect()
}

fn get_hash<S: AsRef<str>>(s: S) -> u8 {
    s.as_ref().as_bytes().iter().fold(0u8, |acc, c| {
        acc.overflowing_add(*c).0.overflowing_mul(17).0
    })
}

#[aoc(day15, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.iter().map(get_hash).map(SolutionType::from).sum()
}

#[aoc(day15, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut boxes: Vec<Vec<(&str, &str)>> = Vec::new();
    for _i in 0..256 {
        boxes.push(Vec::new());
    }

    'instr: for instr in data {
        if let Some((label, focal)) = instr.split_once('=') {
            let hash = get_hash(label);
            let b = &mut boxes[hash as usize];
            for entry in b.iter_mut() {
                if entry.0 == label {
                    entry.1 = focal;
                    continue 'instr;
                }
            }
            b.push((label, focal));
        } else {
            let label = instr.split_once('-').expect("label").0;
            let hash = get_hash(label);
            let b = &mut boxes[hash as usize];
            for i in 0..b.len() {
                if b[i].0 == label {
                    b.remove(i);
                    break;
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(boxnr, cont)| {
            cont.iter()
                .enumerate()
                .map(|(index, (_label, focus))| {
                    usize::from(focus.as_bytes()[0] - b'0') * (boxnr + 1) * (index + 1)
                })
                .sum::<SolutionType>()
        })
        .sum()
}
