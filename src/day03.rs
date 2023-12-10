// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use aoc_runner_derive::{aoc, aoc_generator};

use super::world::*;

type SolutionType = usize;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    // 467..114..
    // ...*......
    Map::from_string(input)
}

fn is_symbol(c: u8) -> bool {
    c != b'.' && !char::from(c).is_ascii_digit()
}

#[aoc(day3, part1)]
pub fn solve_part1(map: &Map) -> SolutionType {
    let mut sum = 0;

    let mut num: SolutionType = 0;
    let mut any_symbol = false;

    for (pos, c) in map.iter() {
        if pos.x == 0 {
            if any_symbol {
                sum += num;
                any_symbol = false;
            }
            num = 0;
        }
        if !char::from(c).is_ascii_digit() {
            if any_symbol {
                sum += num;
            }
            any_symbol = false;
            num = 0;
        } else {
            num = num * 10 + SolutionType::from(c - b'0');
            if !any_symbol {
                for (_pos, _dir, c) in map.neighbors(pos) {
                    any_symbol = is_symbol(c);
                    if any_symbol {
                        break;
                    }
                }
            }
        }
    }
    sum as usize
}

fn add_number(numbers: &mut Vec<SolutionType>, map: &Map, pos: Point) {
    if !char::from(map.get_at(pos)).is_ascii_digit() {
        return;
    }
    let mut pos = map.walk_until(pos, Dir::West, |_, c| !char::from(c).is_ascii_digit());
    let mut sum = 0;
    while map.is_inside_map(pos) && char::from(map.get_at(pos)).is_ascii_digit() {
        sum = sum * 10 + SolutionType::from(map.get_at(pos) - b'0');
        pos = pos.walk(Dir::East);
    }
    numbers.push(sum as SolutionType);
}

#[aoc(day3, part2)]
pub fn solve_part2(map: &Map) -> SolutionType {
    let mut sum = 0;

    for (pos, c) in map.iter() {
        if c == b'*' {
            let mut numbers = vec![];
            add_number(&mut numbers, map, pos.walk(Dir::West));
            add_number(&mut numbers, map, pos.walk(Dir::East));
            if pos.y > 0 {
                if char::from(map.get_at(pos.walk(Dir::North))).is_ascii_digit() {
                    add_number(&mut numbers, map, pos.walk(Dir::North));
                } else {
                    add_number(&mut numbers, map, pos.walk(Dir::NorthWest));
                    add_number(&mut numbers, map, pos.walk(Dir::NorthEast));
                }
            }
            if pos.y < map.get_height() - 1 {
                if char::from(map.get_at(pos.walk(Dir::South))).is_ascii_digit() {
                    add_number(&mut numbers, map, pos.walk(Dir::South));
                } else {
                    add_number(&mut numbers, map, pos.walk(Dir::SouthWest));
                    add_number(&mut numbers, map, pos.walk(Dir::SouthEast));
                }
            }
            if numbers.len() == 2 {
                sum += numbers[0] * numbers[1];
            }
        }
    }
    sum
}
