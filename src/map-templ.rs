// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use super::world::*;
// use std::collections::HashSet;
// use rayon::prelude::*;

type SolutionType = usize;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string(input)
}

#[aoc(day10, part1)]
pub fn solve_part1(map: &Map) -> SolutionType {
    SolutionType::try_from(map.get_width()).expect("answer")
}

#[aoc(day10, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    SolutionType::try_from(map.get_height()).expect("answer")
}
