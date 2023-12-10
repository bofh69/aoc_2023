// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use super::world::*;

type InputType = Map;
type SolutionType = i32;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> InputType {
    // 467..114..
    // ...*......
    Map::from_string_with_border(input)
}

#[aoc(day10, part1)]
pub fn solve_part1(map: &InputType) -> SolutionType {
    map.print();
    map.get_height()
}

#[aoc(day10, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    let mut map: Map = map.clone();
    map.flood_cardinal(Point { x: 3, y: 3 }, b'.', b'A');
    map.flood_cardinal(Point { x: 1, y: 1 }, b'A', b'B');
    map.print();
    /*
    // Game of Life:
    for x in 0..map.get_width() {
        map.set_at(Point{x, y: 0}, b'.');
        map.set_at(Point{x, y: map.get_height() - 1}, b'.');
    }
    for y in 0..map.get_height() {
        map.set_at(Point{x: 0, y}, b'.');
        map.set_at(Point{x: map.get_width() - 1, y}, b'.');
    }
    for gen in 0..10 {
        println!("\nGeneration: {}", gen);
        map.print();
        map.transform(|map, pos, c| {
            let alive = map.neighbors(pos).filter(|(_, _, c)| *c != b'.').count();
            if c == b'.' && alive == 3 {
                b'*'
            } else if c != b'.' && (alive < 2  || alive > 3) {
                b'.'
            } else {
                c
            }
        });
    }

    */
    map.get_width()
}
