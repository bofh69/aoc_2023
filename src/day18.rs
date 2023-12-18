// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use super::world::*;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::*;
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

fn type_from_dirs(from: Dir, to: Dir) -> u8 {
    use Dir::*;
    match (from, to) {
        (North, North) | (South, South) => b'|',
        (East, East) | (West, West) => b'-',
        (South, East) | (West, North) => b'L',
        (North, East) | (West, South) => b'F',
        (North, West) | (East, South) => b'7',
        (South, West) | (East, North) => b'J',
        _ => unreachable!("Should not happen"),
    }
}

fn area_for_y(y: i32, lines: &Vec<(Point, u8, Point, u8)>) -> u64 {
    let mut row: Vec<_> = lines
        .iter()
        .filter(|(from, _, to, _)| from.y <= y && to.y >= y || to.y <= y && from.y >= y)
        .map(|(from, from_type, to, to_type)| {
            let mut result = vec![];
            if from.y == y {
                result.push((*from, *from_type));
            }
            if to.y == y {
                result.push((*to, *to_type));
            }
            if (from.y != to.y) && ((from.y < y && to.y > y) || (from.y > y && to.y < y)) {
                result.push((Point { x: from.x, y }, b'|'));
            }
            result
        })
        .flatten()
        .unique()
        .collect();
    row.sort_by(|(from1, _), (from2, _)| from1.x.cmp(&from2.x));

    println!("For line {}", y);
    println!("{:?}", row);
    let mut sum = 0;
    let mut is_inside = false;
    let mut last_pos = Point { x: 0, y: 0 };
    let mut last_typ = b'.';

    for (pos, typ) in row {
        if typ == b'|' {
            if is_inside {
                sum += pos.x - last_pos.x + 1;
            }
            is_inside = !is_inside;
            last_pos = pos;
            last_typ = typ;
        } else if is_inside {
            match (typ, last_typ) {
                (b'F', _) => {
                }
            }
        } else {
        }
    }
    sum
}

#[aoc(day18, part2)]
pub fn solve_part2(data: &[InputType]) -> SolutionType {
    let mut pos = Point { x: 0, y: 0 };
    let lines: Vec<_> = data
        .iter()
        .map(|(_, _, num)| {
            let line = i32::try_from(num >> 4).expect("number");
            let from = pos;
            let dir = match num & 15 {
                0 => {
                    pos.x += line;
                    Dir::East
                }
                1 => {
                    pos.y += line;
                    Dir::South
                }
                2 => {
                    pos.x -= line;
                    Dir::West
                }
                3 => {
                    pos.y -= line;
                    Dir::North
                }
                _ => unreachable!("Unknown dir"),
            };
            (from, pos, dir)
        })
        .collect();
    let n_lines = lines.len();
    let lines: Vec<_> = lines
        .iter()
        .enumerate()
        .map(|(i, (from, to, dir))| {
            let prev_i = (i + n_lines - 1) % n_lines;
            let prev_dir = lines[prev_i].2;
            let from_type = type_from_dirs(prev_dir, *dir);
            let next_i = (i + 1) % n_lines;
            let next_dir = lines[next_i].2;
            let to_type = type_from_dirs(*dir, next_dir);
            (*from, from_type, *to, to_type)
        })
        .collect();
    println!("{:?}", lines);
    let mut y_pos: Vec<_> = lines
        .iter()
        .map(|(from, _, _, _)| from.y)
        .unique()
        .collect();
    y_pos.sort();
    println!("{:?}", y_pos);
    let mut sum = 0;
    let mut current_area = area_for_y(y_pos[0], &lines);
    let mut last_y = y_pos[0];
    println!("Start area: {}", current_area);
    for &y in y_pos.iter().skip(1) {
        println!("Line {}", y);
        sum += (current_area * u64::try_from(y - last_y).expect("Positive number"))
            as SolutionType;
        let next_area = area_for_y(y, &lines);
        println!("Next area: {}", current_area);
        sum += usize::try_from(current_area.max(next_area)).expect("Positive number");
        println!("Total sum: {}", sum);
        last_y = y;
        current_area = next_area;
    }
    sum
}
