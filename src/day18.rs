// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use super::world::*;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashSet;

type InputType = (u8, u8, u32);
type SolutionType = usize;

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<InputType> {
    // R 4 (#4b18e0)

    let re = Regex::new(r"^(.) ([0-9]*) \(#(.*)\)$").expect("Compilable regex");

    input
        .lines()
        .map(|s| {
            let caps = re
                .captures(s)
                .unwrap_or_else(|| panic!("Didn't match: {}", s));
            (
                caps[1].as_bytes()[0],
                caps[2].parse().expect("number"),
                u32::from_str_radix(&caps[3], 16).expect("hex"),
            )
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn solve_part1(data: &[InputType]) -> SolutionType {
    let mut pos = Point { x: 0, y: 0 };

    let mut points = HashSet::new();
    points.insert(pos);

    for (dir, len, _) in data {
        let dir = match dir {
            b'U' => Dir::North,
            b'D' => Dir::South,
            b'L' => Dir::West,
            b'R' => Dir::East,
            _ => unreachable!("No valid direction"),
        };
        for _i in 0..*len {
            pos = pos.walk(dir);
            points.insert(pos);
        }
    }
    let min_x = points.iter().map(|p| p.x).min().expect("one point");
    let min_y = points.iter().map(|p| p.y).min().expect("one point");
    let max_x = points.iter().map(|p| p.x).max().expect("one point");
    let max_y = points.iter().map(|p| p.y).max().expect("one point");

    let mut map = Map::new(max_x - min_x + 5, max_y - min_y + 5);

    println!("{}, {}", map.get_width(), map.get_height());


    for y in 0..map.get_height() {
        let pos = Point { x: 0, y };
        map.set_at(pos, b'O');
        let pos = Point {
            x: map.get_width() - 1,
            y,
        };
        map.set_at(pos, b'O');
    }
    for x in 0..map.get_width() {
        let pos = Point { x, y: 0 };
        map.set_at(pos, b'O');
        let pos = Point {
            x,
            y: map.get_height() - 1,
        };
        map.set_at(pos, b'O');
    }

    for pos in points {
        let pos = Point {
            x: pos.x + 2 - min_x,
            y: pos.y + 2 - min_y,
        };
        map.set_at(pos, b'#');
    }
    map.flood_cardinal(Point { x: 1, y: 1 }, b'.', b'O');
    map.print();
    SolutionType::try_from(map.get_width() * map.get_height()).expect("Number")
        - map.find(b'O').len()
}

#[aoc(day18, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    data.len()
}
