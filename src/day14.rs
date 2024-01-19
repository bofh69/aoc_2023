// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use super::world::*;
use hashbrown::*;
// use rayon::prelude::*;

type SolutionType = usize;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Map {
    Map::from_string_with_border(input)
}

fn tilt(map: &mut Map, from: Dir, to: Dir) {
    let slide_o = |map: &Map, pos: Point, c| {
        match c {
            b'.' => {
                if map.get_at_unchecked(pos.walk(from)) == b'O' {
                    return b'O';
                }
            }
            b'O' => {
                if map.get_at_unchecked(pos.walk(to)) == b'.' {
                    return b'.';
                }
            }
            _ => (),
        }
        c
    };
    while map.transform(slide_o) {}
}

fn total_value(map: &Map) -> SolutionType {
    let height = map.get_height() - 1;
    map.find(b'O')
        .iter()
        .map(|p| p.y)
        .map(|y| height - y)
        .map(|n| SolutionType::try_from(n).expect("Positive number"))
        .sum()
}

#[aoc(day14, part1)]
pub fn solve_part1(map: &Map) -> SolutionType {
    let mut map = map.clone();
    tilt(&mut map, Dir::South, Dir::North);
    total_value(&map)
}

#[aoc(day14, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    let mut map = map.clone();
    let mut prev = HashMap::new();
    for cycle in 1..1_000 {
        tilt(&mut map, Dir::South, Dir::North);
        tilt(&mut map, Dir::East, Dir::West);
        tilt(&mut map, Dir::North, Dir::South);
        tilt(&mut map, Dir::West, Dir::East);
        if prev.contains_key(&map) {
            let start = prev.get(&map).unwrap();
            let end = 1_000_000_000;
            let loop_length = cycle - start;
            let target = start + (end - start) % loop_length;
            for (m, g) in prev.iter() {
                if *g == target {
                    return total_value(m);
                }
            }
            break;
        } else {
            prev.insert(map.clone(), cycle);
        }
    }
    0
}
