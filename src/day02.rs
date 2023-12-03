// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

type InputType = Vec<(usize, usize, usize)>;
type SolutionType = usize;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    input
        .lines()
        .map(|line| line.split_once(": ").expect("No game separator").1)
        .map(|game| {
            // 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            game.split("; ")
                .map(|round| {
                    // 8 green, 6 blue, 20 red
                    // 5 blue, 4 red, 13 green
                    // 5 green, 1 red
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;
                    for cubes in round.split(", ") {
                        let cubes = cubes
                            .split_once(' ')
                            .expect("space between number and type");
                        let n_cubes: usize = cubes.0.parse().expect("integer");
                        if cubes.1 == "red" {
                            red = n_cubes;
                        } else if cubes.1 == "green" {
                            green = n_cubes;
                        } else if cubes.1 == "blue" {
                            blue = n_cubes;
                        }
                    }
                    (red, green, blue)
                })
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    data.iter()
        .enumerate()
        .filter_map(|(g, rounds)| {
            if rounds.iter().any(|&(r, g, b)| r > 12 || g > 13 || b > 14) {
                None
            } else {
                Some(g + 1)
            }
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.iter()
        .map(|rounds| {
            let min_red = rounds.iter().map(|(r, _g, _b)| *r).max().expect("red");
            let min_green = rounds.iter().map(|(_r, g, _b)| *g).max().expect("green");
            let min_blue = rounds.iter().map(|(_r, _g, b)| *b).max().expect("blue");
            min_red * min_green * min_blue
        })
        .sum()
}
