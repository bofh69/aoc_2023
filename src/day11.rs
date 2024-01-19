// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::aoc;

use super::world::*;
use hashbrown::HashSet;

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let map = Map::<i32>::from_string(input);
    let mut galaxies = map.find(b'#');

    let rows: HashSet<_> = galaxies.iter().map(|p| p.y).collect();
    let cols: HashSet<_> = galaxies.iter().map(|p| p.x).collect();

    for x in 0..map.get_width() {
        if !cols.contains(&x) {
            for pos in galaxies.iter_mut() {
                if pos.x < x {
                    pos.x -= 1;
                }
            }
        }
    }
    for y in 0..map.get_height() {
        if !rows.contains(&y) {
            for pos in galaxies.iter_mut() {
                if pos.y < y {
                    pos.y -= 1;
                }
            }
        }
    }
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            sum += galaxies[i].manhattan_distance(galaxies[j]);
        }
    }
    sum
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let map = Map::<i64>::from_string(input);
    let mut galaxies: Vec<_> = map.find(b'#');

    let rows: HashSet<_> = galaxies.iter().map(|p| p.y).collect();
    let cols: HashSet<_> = galaxies.iter().map(|p| p.x).collect();

    for x in 0..map.get_width() {
        if !cols.contains(&x) {
            for pos in galaxies.iter_mut() {
                if pos.x < x {
                    pos.x -= 999_999;
                }
            }
        }
    }
    for y in 0..map.get_height() {
        if !rows.contains(&y) {
            for pos in galaxies.iter_mut() {
                if pos.y < y {
                    pos.y -= 999_999;
                }
            }
        }
    }
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            sum += galaxies[i].manhattan_distance(galaxies[j]);
        }
    }
    sum
}
