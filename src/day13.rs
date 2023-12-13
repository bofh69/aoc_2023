// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use super::world::*;
// use rayon::prelude::*;

type InputType = String;
type SolutionType = usize;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    // #.#
    // ...
    // ...
    //
    // #..
    // ...
    // #..
    // Max 17x17 characters
    input.split("\n\n").map(|s| s.to_string()).collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|s| Map::<i32>::from_string(s))
        .map(|map| {
            let width = map.get_width();
            let height = map.get_height();
            'next_col: for i in 0..width - 1 {
                for y in 0..height {
                    for x in 0..=i {
                        let from = Point { x, y };
                        let to = Point {
                            x: 2 * (i + 1) - x - 1,
                            y,
                        };
                        if to.x > i && to.x < width && map.get_at(from) != map.get_at(to) {
                            continue 'next_col;
                        }
                    }
                }
                // println!("Found column at {}!", i);
                return SolutionType::try_from(i + 1).expect("Positive number");
            }
            'next_row: for i in 0..height - 1 {
                for y in 0..=i {
                    for x in 0..width {
                        let from = Point { x, y };
                        let to = Point {
                            y: 2 * (i + 1) - y - 1,
                            x,
                        };
                        if to.y > i && to.y < height && map.get_at(from) != map.get_at(to) {
                            continue 'next_row;
                        }
                    }
                }
                // println!("Found row at {}!", i);
                return SolutionType::try_from(100 * (i + 1)).expect("Positive number");
            }
            println!("Didn't find mirror for:");
            map.print();
            0
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|s| Map::<i32>::from_string(s))
        .map(|map| {
            let width = map.get_width();
            let height = map.get_height();
            'next_col: for i in 0..width - 1 {
                // let mut first_smudge = Point{x: 0, y: 0};
                let mut smudges = 0;
                for y in 0..height {
                    for x in 0..=i {
                        let from = Point { x, y };
                        let to = Point {
                            x: 2 * (i + 1) - x - 1,
                            y,
                        };
                        if to.x > i && to.x < width && map.get_at(from) != map.get_at(to) {
                            smudges += 1;
                            // first_smudge = from;
                            if smudges > 1 {
                                continue 'next_col;
                            }
                        }
                    }
                }
                if smudges == 1 {
                    // println!("Found column at {}, smudge at {:?}!", i, first_smudge);
                    return SolutionType::try_from(i + 1).expect("Positive number");
                }
            }
            'next_row: for i in 0..height - 1 {
                // let mut first_smudge = Point{x: 0, y: 0};
                let mut smudges = 0;
                for y in 0..=i {
                    for x in 0..width {
                        let from = Point { x, y };
                        let to = Point {
                            y: 2 * (i + 1) - y - 1,
                            x,
                        };
                        if to.y > i && to.y < height && map.get_at(from) != map.get_at(to) {
                            smudges += 1;
                            // first_smudge = from;
                            if smudges > 1 {
                                continue 'next_row;
                            }
                        }
                    }
                }
                if smudges == 1 {
                    // println!("Found row at {}, smudge at {:?}!", i, first_smudge);
                    return SolutionType::try_from(100 * (i + 1)).expect("Positive number");
                }
            }
            println!("Didn't find mirror for:");
            map.print();
            0
        })
        .sum()
}
