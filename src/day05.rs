// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

type Seed = i64;

pub struct MappingData {
    from_kind: String,
    to_kind: String,
    ranges: Vec<(Seed, Seed, Seed)>,
}

type InputType = (Vec<Seed>, Vec<MappingData>);
type SolutionType = Seed;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> InputType {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .expect("Seeds line")
        .split_once("seeds: ")
        .expect("Seeds")
        .1
        .split_ascii_whitespace()
        .map(|num| num.parse().expect("Seed number"))
        .collect();

    lines.next().expect("empty line");

    let mut mappings = vec![];

    while let Some(mapping) = lines.next() {
        let names = mapping.split_once(" map:").expect("map").0;
        let mut names = names.split('-');
        let from_kind = names.next().expect("from name").to_string();
        names.next();
        let to_kind = names.next().expect("to name").to_string();
        let mut ranges = vec![];
        for range in lines.by_ref() {
            if range.is_empty() {
                break;
            }
            let mut range = range.split_ascii_whitespace();
            let from_range = range
                .next()
                .expect("from range")
                .parse()
                .expect("From range");
            let to_range = range.next().expect("to range").parse().expect("To range");
            let length = range.next().expect("length").parse().expect("Range length");
            ranges.push((from_range, to_range, length));
        }
        mappings.push(MappingData {
            from_kind,
            to_kind,
            ranges,
        });
    }

    (seeds, mappings)
}

fn translate_number(num: Seed, kind: &str, final_kind: &str, data: &InputType) -> SolutionType {
    let mut num = num;
    let mut kind = kind;

    // println!("\n-------");

    while kind != final_kind {
        for trans in &data.1 {
            if trans.from_kind == kind {
                // println!("\nTranslating from {} {}", trans.from_kind, num);
                // let old_num = num;
                kind = trans.to_kind.as_str();
                for range in &trans.ranges {
                    // println!( "Range dest={}, src={}, length={}", range.0, range.1, range.2);
                    if range.1 <= num && (range.1 + range.2) > num {
                        num = num - range.1 + range.0;
                        break;
                    }
                }
                // println!("Translated from {} {} to {} {}", trans.from_kind, old_num, kind, num);
            }
        }
    }
    num
}

#[aoc(day5, part1)]
pub fn solve_part1(data: &InputType) -> SolutionType {
    data.0
        .iter()
        .map(|seed| translate_number(*seed, "seed", "location", data))
        .min()
        .expect("Minimum number")
}

#[aoc(day5, part2)]
pub fn solve_part2(data: &InputType) -> SolutionType {
    let mut numbers = data.0.iter();
    let mut location = SolutionType::MAX;
    while let Some(start) = numbers.next() {
        let size = numbers.next().expect("seed size");
        for i in *start..(start + size) {
            let min = translate_number(i, "seed", "location", data);
            if min < location {
                println!("New min {}", min);
                location = min;
            }
        }
    }
    location
}
